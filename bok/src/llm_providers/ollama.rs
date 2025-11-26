use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use crate::config::LlmSettings;
use crate::llm::LlmProvider;

#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
}

pub struct OllamaProvider {
    settings: LlmSettings,
    client: Client,
    runtime: Runtime,
}

impl OllamaProvider {
    pub fn new(settings: LlmSettings) -> Self {
        OllamaProvider {
            settings,
            client: Client::new(),
            runtime: Runtime::new().unwrap(),
        }
    }
}

impl LlmProvider for OllamaProvider {
    fn dissect_markdown(&self, markdown_content: &str) -> Result<Vec<(String, String)>> {
        let ollama_url = self.settings.location.as_deref().unwrap_or("http://localhost:11434");
        let chat_url = format!("{}/api/chat", ollama_url);

        let system_prompt = "You are a helpful assistant that dissects markdown content into its smallest semantic units. For each unit, provide a short blurb (up to 50 characters) and the full content of the unit. Respond with a JSON array of arrays, where each inner array contains two strings: [\"blurb\", \"dissected_unit_content\"]. Do not include any other text in your response.";
        let user_prompt = format!("Dissect the following markdown:\n\n{}", markdown_content);

        let request_body = OllamaChatRequest {
            model: self.settings.model.clone(),
            messages: vec![
                OllamaMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OllamaMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            stream: false,
        };

        let response = self.runtime.block_on(async {
            self.client.post(&chat_url)
                .json(&request_body)
                .send()
                .await
        })?;

        let response_text = self.runtime.block_on(async {
            response.text().await
        })?;
        
        let ollama_response: OllamaChatResponse = serde_json::from_str(&response_text)?;

        // Assuming Ollama returns a JSON array of [blurb, content] arrays directly in the content
        let dissected_parts: Vec<(String, String)> = serde_json::from_str(&ollama_response.message.content)?;

        Ok(dissected_parts)
    }
}
