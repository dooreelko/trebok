use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LlmSettings {
    pub provider: String,
    pub model: String,
    pub location: Option<String>,
    pub port: Option<u16>,
}

impl Default for LlmSettings {
    fn default() -> Self {
        LlmSettings {
            provider: "ollama".to_string(),
            model: "qwen3:8b".to_string(),
            location: Some("http://localhost".to_string()),
            port: Some(11434),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FullConfig {
    #[serde(default)]
    pub llm: LlmSettings,
    pub title: Option<String>,
    pub author: Option<String>,
    pub starting_node: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub llm: LlmSettings,
}

impl Settings {
    pub fn new() -> Result<Self, anyhow::Error> {
        let config_path = PathBuf::from("bok.yaml");

        if !config_path.exists() {
            eprintln!("Error: bok.yaml not found. Please run 'bok init' to create it.");
            anyhow::bail!("bok.yaml not found");
        }

        let file_content = fs::read_to_string(config_path)?;
        let config: FullConfig = serde_yaml::from_str(&file_content)?;

        Ok(Settings { llm: config.llm })
    }
}
