use crate::config::LlmSettings;
use crate::llm_providers::ollama::{OllamaProvider, OllamaStream};
use anyhow::{Result, anyhow};
use async_trait::async_trait;

#[async_trait]
pub trait AsyncIterator {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}

#[async_trait]
pub trait LlmProvider {
    type Iterator<'a>: AsyncIterator<Item = Result<(String, String)>> + Send + 'a
    where
        Self: 'a;

    async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Self::Iterator<'a>>;
}

pub struct DummyLlmProvider;

pub struct DummyIterator<'a> {
    content: Vec<&'a str>,
    index: usize,
}

impl<'a> DummyIterator<'a> {
    fn new(content: &'a str) -> Self {
        Self {
            content: content.split("\n\n").collect(),
            index: 0,
        }
    }
}

#[async_trait]
impl<'a> AsyncIterator for DummyIterator<'a> {
    type Item = Result<(String, String)>;

    async fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.content.len() {
            return None;
        }

        let s = self.content[self.index];
        self.index += 1;

        let blurb = s.chars().take(50).collect::<String>();
        Some(Ok((blurb, s.to_string())))
    }
}

#[async_trait]
impl LlmProvider for DummyLlmProvider {
    type Iterator<'a> = DummyIterator<'a>;

    async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<Self::Iterator<'a>> {
        Ok(DummyIterator::new(markdown_content))
    }
}

pub enum Llm {
    Ollama(OllamaProvider),
    Dummy(DummyLlmProvider),
}

pub enum LlmIterator<'a> {
    Ollama(
        OllamaStream<
            std::pin::Pin<
                Box<dyn futures::Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send>,
            >,
        >,
    ),
    Dummy(DummyIterator<'a>),
}

#[async_trait]
impl<'a> AsyncIterator for LlmIterator<'a> {
    type Item = Result<(String, String)>;

    async fn next(&mut self) -> Option<Self::Item> {
        match self {
            LlmIterator::Ollama(iter) => iter.next().await,
            LlmIterator::Dummy(iter) => iter.next().await,
        }
    }
}

impl Llm {
    pub async fn dissect_markdown<'a>(
        &'a self,
        markdown_content: &'a str,
    ) -> Result<LlmIterator<'a>> {
        match self {
            Llm::Ollama(provider) => {
                let iterator = provider.dissect_markdown(markdown_content).await?;
                Ok(LlmIterator::Ollama(iterator))
            }
            Llm::Dummy(provider) => {
                let iterator = provider.dissect_markdown(markdown_content).await?;
                Ok(LlmIterator::Dummy(iterator))
            }
        }
    }
}

pub fn get_llm_provider(settings: &LlmSettings) -> Result<Llm> {
    match settings.provider.as_str() {
        "ollama" => {
            println!("Using Ollama provider.");
            Ok(Llm::Ollama(OllamaProvider::new(settings.clone())))
        }
        "dummy" => {
            println!("Using Dummy provider.");
            Ok(Llm::Dummy(DummyLlmProvider))
        }
        "anthropic" => {
            // TODO: Implement AnthropicProvider
            println!("Using Anthropic provider (dummy implementation)");
            Ok(Llm::Dummy(DummyLlmProvider))
        }
        "openai" => {
            // TODO: Implement OpenAIProvider
            println!("Using OpenAI provider (dummy implementation)");
            Ok(Llm::Dummy(DummyLlmProvider))
        }
        _ => Err(anyhow!("Unknown LLM provider: {}", settings.provider)),
    }
}
