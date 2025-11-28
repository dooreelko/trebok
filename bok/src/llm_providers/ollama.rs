use std::io;
use std::pin::Pin;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use bytes::Bytes;
use futures::TryStreamExt;
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::io::StreamReader;

use crate::config::LlmSettings;
use crate::llm::{AsyncIterator, LlmProvider};

#[derive(Debug)]
pub struct OllamaProvider {
    pub settings: LlmSettings,
}

impl OllamaProvider {
    pub fn new(settings: LlmSettings) -> Self {
        OllamaProvider { settings }
    }
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    stream: bool,
    think: bool,
    prompt: &'a str,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    #[serde(default)]
    response: Option<String>,
}

pub struct OllamaStream<S>
where
    S: Stream<Item = Result<Bytes, io::Error>> + Unpin + Send,
{
    reader: BufReader<StreamReader<S, Bytes>>,
    buffer: String,
}

#[async_trait]
impl<S> AsyncIterator for OllamaStream<S>
where
    S: Stream<Item = Result<Bytes, io::Error>> + Unpin + Send,
{
    type Item = Result<(String, String)>;

    async fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Read next line from the stream (each line is a GenerateResponse JSON)
            let mut line = String::new();
            match self.reader.read_line(&mut line).await {
                Ok(0) => {
                    // End of stream
                    return None;
                }
                Ok(_) => {
                    // Parse the line as GenerateResponse
                    match serde_json::from_str::<GenerateResponse>(&line) {
                        Ok(gen_response) => {
                            if let Some(response_text) = gen_response.response {
                                self.buffer.push_str(&response_text);

                                // Check if this response contains a newline
                                if response_text.contains('\n') || response_text.contains('\r') {
                                    // We have a complete line, parse it
                                    if let Some(newline_pos) = self.buffer.find('\n') {
                                        let complete_line = self.buffer[..newline_pos]
                                            .trim_end_matches('\r')
                                            .to_string();
                                        self.buffer.drain(..=newline_pos);

                                        // Parse as JSON array ["blurb", "content"]
                                        match serde_json::from_str::<Vec<String>>(&complete_line) {
                                            Ok(parts) if parts.len() == 2 => {
                                                return Some(Ok((
                                                    parts[0].clone(),
                                                    parts[1].clone(),
                                                )));
                                            }
                                            Ok(_) => {
                                                return Some(Err(anyhow!(
                                                    "Expected JSON array with 2 elements"
                                                )));
                                            }
                                            Err(e) => {
                                                return Some(Err(anyhow!(
                                                    "Failed to parse JSON array: {}",
                                                    e
                                                )));
                                            }
                                        }
                                    }
                                }
                            }
                            // Continue reading more responses
                        }
                        Err(e) => {
                            return Some(Err(anyhow!("Failed to parse GenerateResponse: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    return Some(Err(anyhow!("IO error reading stream: {}", e)));
                }
            }
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    type Iterator<'a> = OllamaStream<Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>>;

    async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Self::Iterator<'a>> {
        let ollama_url = self
            .settings
            .location
            .as_deref()
            .unwrap_or("http://localhost")
            .to_string();
        let ollama_port = self.settings.port.unwrap_or(11434);
        let full_url = format!("{}:{}/api/generate", ollama_url, ollama_port);

        let system_prompt = r#"You are a helpful assistant that dissects markdown content into its smallest atomic semantic units. For each unit, provide a short blurb (up to 50 characters) in form of "subject verb characteristic" and the full content of the unit. For example for text 'Here's the idea. I'm not thinking in a linear manner and (hypothesis) I think this hinders my attempts to write my book. The idea is to create a recursive mind-map-like graph system, where each node  represents and idea or a concept and can have its own sub-graph of concepts that will elaborate the parent concept further.' the blurb could be 'non-linear thinking requires specialized tooling'. For each unit respond with a one-line JSON array that contains two strings: ["blurb", "dissected_unit_content"] followed by a new line. Do not include any other text in your response, just one two-item array per line."#;
        let user_prompt = format!("Dissect the following markdown:\n\n{}", markdown_content);

        let client = Client::new();
        let model_name: &str = self.settings.model.as_str();
        let prompt = format!("{} {}", system_prompt, user_prompt);
        eprintln!("Using {}", model_name);
        let request_body = serde_json::to_string(&ChatRequest {
            model: model_name,
            stream: true,
            think: false,
            prompt: &prompt,
        })?;

        let res = client
            .post(&full_url)
            .header("Content-Type", "application/json")
            .body(request_body)
            .send()
            .await?
            .error_for_status()?;

        let byte_stream = res
            .bytes_stream()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));
        let stream_reader =
            StreamReader::new(Box::pin(byte_stream)
                as Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>);
        let reader = BufReader::new(stream_reader);

        Ok(OllamaStream {
            reader,
            buffer: String::new(),
        })
    }
}
