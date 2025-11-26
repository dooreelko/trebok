use crate::config::LlmSettings;
use crate::llm_providers::ollama::OllamaProvider;
use anyhow::{Result, anyhow};

pub trait LlmProvider {
    fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Result<(String, String)>> + 'a>>;
}

pub struct DummyLlmProvider;

impl LlmProvider for DummyLlmProvider {
    fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Box<dyn Iterator<Item = Result<(String, String)>> + 'a>> {
        let iterator = markdown_content.split("\n\n").map(|s| {
            let blurb = s.chars().take(50).collect::<String>();
            Ok((blurb, s.to_string()))
        });
        Ok(Box::new(iterator))
    }
}

pub fn get_llm_provider(settings: &LlmSettings) -> Result<Box<dyn LlmProvider>> {
    match settings.provider.as_str() {
        "ollama" => {
            println!("Using Ollama provider.");
            Ok(Box::new(OllamaProvider::new(settings.clone())))
        }
        "dummy" => {
            println!("Using Dummy provider.");
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
