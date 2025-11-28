use crate::config::LlmSettings;
use crate::llm::LlmProvider;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::{self, Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Debug)]
pub struct OllamaProvider {
    settings: LlmSettings,
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
struct ChatResponse {
    message: Option<MessageResponse>,
    done: bool,
    #[serde(default)]
    done_reason: Option<String>,
    #[serde(default)]
    total_duration: Option<u64>,
    #[serde(default)]
    load_duration: Option<u64>,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    prompt_eval_duration: Option<u64>,
    #[serde(default)]
    eval_count: Option<u32>,
    #[serde(default)]
    eval_duration: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    #[serde(default)]
    response: Option<String>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    thinking: Option<String>,
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<(String, String)>> + Send + 'a>>> {
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
        let request_body = ChatRequest {
            model: model_name,
            stream: true,
            think: false,
            prompt: prompt.as_str(),
        };

        let res = client
            .post(&full_url)
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        let byte_stream = res.bytes_stream();

        let stream = stream::unfold(
            (byte_stream, String::new()),
            async move |(mut byte_stream, mut buffer)| {
                loop {
                    let chunk_result = byte_stream.next().await;
                    let chunk = match chunk_result {
                        Some(Ok(c)) => c,
                        Some(Err(e)) => {
                            return Some((Err(anyhow::Error::new(e)), (byte_stream, buffer)));
                        }
                        None => {
                            // If the stream ended, process any remaining content in the buffer
                            if !buffer.is_empty() {
                                let line = buffer.trim().to_string();
                                buffer.clear(); // Clear buffer after processing
                                if line.is_empty() {
                                    return None;
                                }
                                let parsed_response: serde_json::Value =
                                    match serde_json::from_str(&line) {
                                        Ok(res) => res,
                                        Err(e) => {
                                            return Some((
                                                Err(anyhow::Error::new(e)),
                                                (byte_stream, buffer),
                                            ));
                                        }
                                    };
                                if let Some(arr) = parsed_response.as_array() {
                                    if arr.len() == 2 {
                                        if let (Some(blurb), Some(content)) =
                                            (arr[0].as_str(), arr[1].as_str())
                                        {
                                            return Some((
                                                Ok((blurb.to_string(), content.to_string())),
                                                (byte_stream, buffer),
                                            ));
                                        }
                                    }
                                }
                            }
                            return None; // Stream ended and buffer processed
                        }
                    };

                    let generate_response: GenerateResponse =
                        match serde_json::from_str(&String::from_utf8_lossy(&chunk)) {
                            Ok(res) => res,
                            Err(e) => {
                                return Some((Err(anyhow::Error::new(e)), (byte_stream, buffer)));
                            }
                        };

                    if let Some(res) = generate_response.response {
                        buffer.push_str(&res);

                        let mut lines_to_process = Vec::new();
                        let mut new_buffer = String::new();

                        let mut split = buffer.split('\n').peekable();
                        while let Some(line) = split.next() {
                            if split.peek().is_some() {
                                // It's a complete line
                                lines_to_process.push(line.to_string());
                            } else {
                                // It's the last (potentially incomplete) line
                                new_buffer.push_str(line);
                            }
                        }
                        buffer = new_buffer; // Update buffer with the incomplete part

                        for line in lines_to_process {
                            let line = line.trim();
                            if line.is_empty() {
                                continue;
                            }

                            let parsed_response: serde_json::Value =
                                match serde_json::from_str(&line) {
                                    Ok(res) => res,
                                    Err(e) => {
                                        return Some((
                                            Err(anyhow::Error::new(e)),
                                            (byte_stream, buffer),
                                        ));
                                    }
                                };

                            if let Some(arr) = parsed_response.as_array() {
                                if arr.len() == 2 {
                                    if let (Some(blurb), Some(content)) =
                                        (arr[0].as_str(), arr[1].as_str())
                                    {
                                        return Some((
                                            Ok((blurb.to_string(), content.to_string())),
                                            (byte_stream, buffer),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    if generate_response.done {
                        // Process any remaining content in the buffer when done
                        if !buffer.is_empty() {
                            let line = buffer.trim().to_string();
                            buffer.clear(); // Clear buffer after processing
                            if line.is_empty() {
                                return None;
                            }
                            let parsed_response: serde_json::Value =
                                match serde_json::from_str(&line) {
                                    Ok(res) => res,
                                    Err(e) => {
                                        return Some((
                                            Err(anyhow::Error::new(e)),
                                            (byte_stream, buffer),
                                        ));
                                    }
                                };
                            if let Some(arr) = parsed_response.as_array() {
                                if arr.len() == 2 {
                                    if let (Some(blurb), Some(content)) =
                                        (arr[0].as_str(), arr[1].as_str())
                                    {
                                        return Some((
                                            Ok((blurb.to_string(), content.to_string())),
                                            (byte_stream, buffer),
                                        ));
                                    }
                                }
                            }
                        }
                        return None;
                    }
                }
            },
        );

        Ok(Box::pin(stream))
    }
}
