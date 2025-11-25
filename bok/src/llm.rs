use crate::config::LlmSettings;
use anyhow::{Result, anyhow};

pub trait LlmProvider {
    fn dissect_markdown(&self, markdown_content: &str) -> Result<Vec<String>>;
}

pub struct DummyLlmProvider;

impl LlmProvider for DummyLlmProvider {
    fn dissect_markdown(&self, markdown_content: &str) -> Result<Vec<String>> {
        // For now, just split by double newline as a dummy implementation
        Ok(markdown_content
            .split("\n\n")
            .map(|s| s.to_string())
            .collect())
    }
}

pub fn get_llm_provider(settings: &LlmSettings) -> Result<Box<dyn LlmProvider>> {
    match settings.provider.as_str() {
        "ollama" => {
            // TODO: Implement OllamaProvider
            println!("Using Ollama provider (dummy implementation)");
            Ok(Box::new(DummyLlmProvider))
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
