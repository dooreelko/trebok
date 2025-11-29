use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Metadata structure for a node, stored in meta.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// The node's title/blurb
    pub title: String,
    /// Optional ID of sibling node that should precede this one (for ordering)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

/// Wrapper structure for the meta.yaml file format
#[derive(Debug, Serialize, Deserialize)]
struct MetaFile {
    meta: Meta,
}

impl Meta {
    pub fn new(title: String, after: Option<String>) -> Self {
        Meta { title, after }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub meta: Meta,
    pub children: Vec<Node>,
}

impl Node {
    /// Accessor for blurb (delegates to meta.title)
    pub fn blurb(&self) -> &str {
        &self.meta.title
    }

    /// Accessor for after (delegates to meta.after)
    pub fn after(&self) -> Option<&String> {
        self.meta.after.as_ref()
    }
}

impl Node {
    pub fn new(id: String, blurb: String, after: Option<String>) -> Self {
        Node {
            id,
            meta: Meta::new(blurb, after),
            children: Vec::new(),
        }
    }

    /// Load node from meta.yaml file, deserializing the entire structure
    pub fn from_meta(meta_path: &Path, id: String) -> Result<Self, String> {
        let content = fs::read_to_string(meta_path)
            .map_err(|e| format!("Failed to read meta.yaml: {}", e))?;

        let meta_file: MetaFile = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse meta.yaml: {}", e))?;

        Ok(Node {
            id,
            meta: meta_file.meta,
            children: Vec::new(),
        })
    }

    /// Save node metadata to meta.yaml file
    pub fn save_meta(&self, meta_path: &Path) -> Result<(), String> {
        let meta_file = MetaFile {
            meta: self.meta.clone(),
        };

        let yaml_content = serde_yaml::to_string(&meta_file)
            .map_err(|e| format!("Failed to serialize meta to YAML: {}", e))?;

        fs::write(meta_path, yaml_content).map_err(|e| format!("Failed to write meta.yaml: {}", e))
    }
}
