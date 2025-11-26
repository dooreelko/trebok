use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct LlmSettings {
    pub provider: String,
    pub model: String,
    pub location: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub llm: LlmSettings,
}

impl Settings {
    pub fn new() -> Result<Self, hocon::Error> {
        let default_config = r#"
            llm {
                provider = "ollama"
                model = "qwen3:8b"
                location = "http://localhost"
                port = 11434
            }
            title = "My New Book"
            author = "Unknown Author"
            starting_node = "" 
        "#;

        let config_path = PathBuf::from("bok.hocon");

        let config_str = if config_path.exists() {
            let file_content = fs::read_to_string(config_path)?;
            format!("{}\n{}", default_config, file_content)
        } else {
            eprintln!("Error: bok.hocon not found. Please run 'bok init' to create it.");
            return Err(hocon::Error::KeyNotFound {
                key: "bok.hocon".to_string(),
            });
        };

        let settings: Settings = hocon::de::from_str(&config_str)?;

        Ok(settings)
    }
}
