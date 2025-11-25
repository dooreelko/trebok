use config::{ConfigError};
use serde::Deserialize;
use std::path::PathBuf;
use hocon::HoconLoader;

#[derive(Debug, Deserialize, Clone)]
pub struct LlmSettings {
    pub provider: String,
    pub model: String,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub llm: LlmSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = PathBuf::from("bok.conf");
        if !config_path.exists() {
            eprintln!(
                "Warning: bok.conf not found in the current directory. Using default settings."
            );
            // Return a default settings if the file doesn't exist
            return Ok(Settings {
                llm: LlmSettings {
                    provider: "ollama".to_string(),
                    model: "qwen3:14b".to_string(),
                    location: Some("http://localhost:11434".to_string()),
                },
            });
        }

        let hocon = HoconLoader::new()
            .load_file(config_path)
            .map_err(|e| ConfigError::Foreign(Box::new(e)))?
            .hocon()
            .map_err(|e| ConfigError::Foreign(Box::new(e)))?;

        // Convert Hocon to a format that can be deserialized by serde
        // This is a bit of a workaround as hocon crate doesn't directly integrate with config crate's sources
        // For simplicity, we'll manually extract and build the Settings struct
        let llm_provider = hocon["llm"]["provider"]
            .as_string()
            .ok_or_else(|| ConfigError::Message("LLM provider not found in bok.conf".to_string()))?;
        let llm_model = hocon["llm"]["model"]
            .as_string()
            .ok_or_else(|| ConfigError::Message("LLM model not found in bok.conf".to_string()))?;
        let llm_location = hocon["llm"]["location"].as_string();

        Ok(Settings {
            llm: LlmSettings {
                provider: llm_provider,
                model: llm_model,
                location: llm_location,
            },
        })
    }
}
