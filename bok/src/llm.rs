use crate::config::LlmSettings;
use anyhow::{Result, anyhow};
use crate::llm_providers::ollama::OllamaProvider;

pub trait LlmProvider {
    fn dissect_markdown(&self, markdown_content: &str) -> Result<Vec<(String, String)>>;
}

pub struct DummyLlmProvider;

impl LlmProvider for DummyLlmProvider {
    fn dissect_markdown(&self, markdown_content: &str) -> Result<Vec<(String, String)>> {
        Ok(markdown_content
            .split("\n\n")
            .map(|s| {
                let blurb = s.chars().take(50).collect::<String>();
                (blurb, s.to_string())
            })
            .collect())
    }
}

pub fn get_llm_provider(settings: &LlmSettings) -> Result<Box<dyn LlmProvider>> {
    match settings.provider.as_str() {
        "ollama" => {
            println!("Using Ollama provider.");
            Ok(Box::new(OllamaProvider::new(settings.clone())))
        }
        "anthropic" => {
            // TODO: Implement AnthropicProvider
            println!("Using Anthropic provider (dummy implementation)");
            Ok(Box::new(DummyLlmProvider))
        }
        "openai" => {
            // TODO: Implement OpenAIProvider
            println!("Using OpenAI provider (dummy implementation)");
            Ok(Box::new(DummyLlmProvider))
        }
        _ => Err(anyhow!("Unknown LLM provider: {}", settings.provider)),
    }
}
