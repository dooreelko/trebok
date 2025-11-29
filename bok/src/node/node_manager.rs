use super::node::Node;
use glob::glob;
use murmur3::murmur3_32;
use serde::Deserialize;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct BokConfig {
    starting_node: String,
}

pub struct NodeManager;

impl NodeManager {
    /// Find a node by its hash/id and return its path
    pub fn find_node_path(node_hash: &str) -> Result<PathBuf, String> {
        let pattern = format!("./**/{}*", node_hash);
        for entry in glob(&pattern).map_err(|e| format!("Failed to read glob pattern: {}", e))? {
            if let Ok(p) = entry {
                if p.is_dir() {
                    return Ok(p);
                }
            }
        }
        Err(format!("Node with hash '{}' not found.", node_hash))
    }

    /// Find a node by its hash/id with a specific pattern
    pub fn find_node_path_with_pattern(pattern: &str) -> Result<PathBuf, String> {
        for entry in glob(pattern).map_err(|e| format!("Failed to read glob pattern: {}", e))? {
            if let Ok(p) = entry {
                if p.is_dir() {
                    return Ok(p);
                }
            }
        }
        Err("Node not found with pattern".to_string())
    }

    /// Get the content of a node's text.qmd file
    pub fn get_node_content(node_hash: &str) -> Result<String, String> {
        let node_path = Self::find_node_path(node_hash)?;
        let text_file_path = node_path.join("text.qmd");
        fs::read_to_string(&text_file_path)
            .map_err(|e| format!("Error reading text.qmd for node '{}': {}", node_hash, e))
    }

    /// Create a new node with the given blurb, content, and optional parent
    pub fn create_node(
        blurb: &str,
        content: &str,
        under: Option<&str>,
        after: Option<&str>,
    ) -> Result<u32, String> {
        let mut reader = Cursor::new(blurb.as_bytes());
        let node_id = murmur3_32(&mut reader, 0).unwrap();
        let node_dir_name = format!("{} {}", node_id, blurb);

        let mut path = PathBuf::new();
        if let Some(under_hash) = under {
            let parent_pattern = format!("./{} *", under_hash);
            let parent_path = Self::find_node_path_with_pattern(&parent_pattern)
                .map_err(|_| format!("Parent node with hash {} not found.", under_hash))?;
            path.push(parent_path);
        }
        path.push(node_dir_name);

        fs::create_dir_all(&path).unwrap();

        let text_file_path = path.join("text.qmd");
        fs::write(text_file_path, content).unwrap();

        let meta_file_path = path.join("meta.yaml");
        let node = Node::new(
            node_id.to_string(),
            blurb.to_string(),
            after.map(String::from),
        );
        node.save_meta(&meta_file_path)?;

        Ok(node_id)
    }

    /// Remove a node by its hash
    pub fn remove_node(node_hash: &str) -> Result<PathBuf, String> {
        let node_path = Self::find_node_path(node_hash)?;
        fs::remove_dir_all(&node_path)
            .map_err(|e| format!("Error removing node '{}': {}", node_path.display(), e))?;
        Ok(node_path)
    }

    /// Build node tree recursively from filesystem
    pub fn get_nodes_recursive(dir: &Path) -> Vec<Node> {
        let mut nodes = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            let entries_vec: Vec<_> = entries.filter_map(Result::ok).collect();

            for entry in entries_vec {
                let path = entry.path();
                if path.is_dir() {
                    let meta_path = path.join("meta.yaml");
                    if meta_path.exists() {
                        let node_name = path.file_name().unwrap().to_str().unwrap();
                        let parts: Vec<&str> = node_name.splitn(2, ' ').collect();
                        if parts.len() == 2 {
                            let id = parts[0].to_string();

                            match Node::from_meta(&meta_path, id) {
                                Ok(mut node) => {
                                    node.children = Self::get_nodes_recursive(&path);
                                    nodes.push(node);
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Warning: Failed to load node from {}: {}",
                                        meta_path.display(),
                                        e
                                    );
                                }
                            }
                        }
                    }
                }
            }

            // Sort nodes according to "after" ordering
            Self::sort_by_after_attribute(&mut nodes);
        }
        nodes
    }

    /// Topological sort based on "after" attribute
    pub fn sort_by_after_attribute(nodes: &mut Vec<Node>) {
        let mut sorted = Vec::new();
        let mut remaining: Vec<_> = nodes.drain(..).collect();

        while !remaining.is_empty() {
            let mut progress = false;

            for i in (0..remaining.len()).rev() {
                let node = &remaining[i];

                // Check if this node can be placed
                let can_place = match node.after() {
                    None => true, // No dependency, can be placed anytime
                    Some(after_id) => {
                        // Check if the node we depend on is already in sorted
                        sorted.iter().any(|n: &Node| n.id == *after_id)
                    }
                };

                if can_place {
                    let node = remaining.remove(i);
                    sorted.push(node);
                    progress = true;
                }
            }

            // If no progress was made, we have a circular dependency or missing reference
            // Just add remaining nodes in their current order
            if !progress && !remaining.is_empty() {
                sorted.extend(remaining.drain(..));
                break;
            }
        }

        *nodes = sorted;
    }

    /// Get all nodes as a flat list (id, blurb pairs)
    pub fn get_all_nodes_flat() -> Vec<(String, String)> {
        let mut nodes_flat = Vec::new();
        Self::get_all_nodes_flat_recursive(Path::new("."), &mut nodes_flat);
        nodes_flat
    }

    /// Helper for get_all_nodes_flat
    fn get_all_nodes_flat_recursive(dir: &Path, nodes_flat: &mut Vec<(String, String)>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let meta_path = path.join("meta.yaml");
                    if meta_path.exists() {
                        let node_name = path.file_name().unwrap().to_str().unwrap();
                        let parts: Vec<&str> = node_name.splitn(2, ' ').collect();
                        if parts.len() == 2 {
                            let id = parts[0].to_string();
                            let blurb = parts[1].to_string();
                            nodes_flat.push((id, blurb));
                            Self::get_all_nodes_flat_recursive(&path, nodes_flat);
                        }
                    }
                }
            }
        }
    }

    /// Print nodes recursively with indentation
    pub fn print_nodes_recursive(nodes: &[Node], prefix: &str) {
        for node in nodes {
            println!("{}{} {}", prefix, node.id, node.blurb());
            Self::print_nodes_recursive(&node.children, &format!("  {}", prefix));
        }
    }

    /// List nodes starting from the configured starting node
    pub fn list_nodes() -> Result<(), String> {
        let content = fs::read_to_string("bok.yaml")
            .map_err(|e| format!("Unable to load bok.yaml: {}", e))?;

        let bok_config: BokConfig = serde_yaml::from_str(&content)
            .map_err(|e| format!("Unable to parse bok.yaml: {}", e))?;

        let starting_node_id = bok_config.starting_node;

        let mut all_nodes = Self::get_nodes_recursive(Path::new("."));

        let mut starting_node_index = None;
        for (i, node) in all_nodes.iter().enumerate() {
            if node.id == starting_node_id {
                starting_node_index = Some(i);
                break;
            }
        }

        if let Some(index) = starting_node_index {
            let starting_node = all_nodes.remove(index);
            println!("{} {}", starting_node.id, starting_node.blurb());
            Self::print_nodes_recursive(&starting_node.children, "  ");
        } else {
            eprintln!("Starting node with id {} not found.", starting_node_id);
        }

        // Print remaining root nodes
        for node in all_nodes {
            println!("{} {}", node.id, node.blurb());
            Self::print_nodes_recursive(&node.children, "  ");
        }

        Ok(())
    }
}
