use hocon::HoconLoader;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Metadata structure for a node, stored in meta.hocon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    /// The node's title/blurb
    pub title: String,
    /// Optional ID of sibling node that should precede this one (for ordering)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
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

    /// Load node from meta.hocon file, deserializing the entire structure
    pub fn from_meta(meta_path: &Path, id: String) -> Result<Self, String> {
        let hocon = HoconLoader::new()
            .load_file(meta_path)
            .map_err(|e| format!("Failed to load meta.hocon: {}", e))?
            .hocon()
            .map_err(|e| format!("Failed to parse meta.hocon: {}", e))?;

        // Extract fields from HOCON meta section
        let meta_section = &hocon["meta"];

        let title = meta_section["title"]
            .as_string()
            .ok_or("Missing 'meta.title' field in meta.hocon")?;

        let after = meta_section["after"].as_string();

        let meta = Meta::new(title, after);

        Ok(Node {
            id,
            meta,
            children: Vec::new(),
        })
    }

    /// Save node metadata to meta.hocon file
    pub fn save_meta(&self, meta_path: &Path) -> Result<(), String> {
        let mut meta_content = String::from("meta {\n");
        meta_content.push_str(&format!(r#"  title: "{}""#, self.meta.title));
        meta_content.push_str("\n");
        if let Some(ref after_id) = self.meta.after {
            meta_content.push_str(&format!(r#"  after: "{}""#, after_id));
            meta_content.push_str("\n");
        }
        meta_content.push_str("}");

        fs::write(meta_path, meta_content).map_err(|e| format!("Failed to write meta.hocon: {}", e))
    }
}
