use crate::config::LlmSettings;
use crate::llm::LlmProvider;
use anyhow::{Result, anyhow};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{Write, stderr};

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

impl LlmProvider for OllamaProvider {
    fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Result<(String, String)>> + 'a>> {
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

        let mut results = Vec::new();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(async {
            let res = client
                .post(&full_url)
                .json(&request_body)
                .send()
                .await?
                .error_for_status()?;

            let mut buffer = String::new();
            let mut byte_stream = res.bytes_stream();

            while let Some(chunk_result) = byte_stream.next().await {
                let chunk = chunk_result?;
                let generate_response: GenerateResponse = serde_json::from_str(&String::from_utf8_lossy(&chunk))?;

                if let Some(res) = generate_response.response {
                    buffer.push_str(&res);
                    let pos = buffer.find('\n').unwrap_or(usize::MAX);

                    if generate_response.done || pos != usize::MAX {
                        let line = buffer.clone();
                        buffer.clear();
                        let line = line.trim();

                        eprintln!("Got one {} {} {}, {}", line, generate_response.done, pos, pos != usize::MAX);
    
                        if line.is_empty() {
                            continue;
                        }
                        let parsed_response: serde_json::Value =
                            serde_json::from_str(&line)?;

                        if let Some(arr) = parsed_response.as_array() {
                            if arr.len() == 2 {
                                if let (Some(blurb), Some(content)) =
                                    (arr[0].as_str(), arr[1].as_str())
                                {
                                    results.push(Ok((blurb.to_string(), content.to_string())));
                                }
                            }
                        }
                    }
                }

            }

            Ok::<(), anyhow::Error>(())
        })?;

        Ok(Box::new(results.into_iter()))
    }
}
