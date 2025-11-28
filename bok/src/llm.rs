use crate::config::LlmSettings;
use crate::llm_providers::ollama::OllamaProvider;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

#[async_trait]
pub trait LlmProvider {
    async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<(String, String)>> + Send + 'a>>>;
}

pub struct DummyLlmProvider;

#[async_trait]
impl LlmProvider for DummyLlmProvider {
    async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<(String, String)>> + Send + 'a>>> {
        let iterator = markdown_content.split("\n\n").map(|s| {
            let blurb = s.chars().take(50).collect::<String>();
            Ok((blurb, s.to_string()))
        });
        let stream = futures::stream::iter(iterator);
        Ok(Box::pin(stream))
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
