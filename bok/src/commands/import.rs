use crate::commands::node;
use crate::config::Settings;
use crate::llm::{get_llm_provider};
use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

pub fn run(file: &str, under: Option<&str>) -> Result<()> {
    let file_path = Path::new(file);

    if !file_path.exists() {
        eprintln!("Error: File '{}' not found.", file);
        return Ok(());
    }

    let original_content = fs::read_to_string(file_path)?;

    let settings = Settings::new()?;
    let llm_provider = get_llm_provider(&settings.llm)?;

    let parts = llm_provider.dissect_markdown(&original_content)?;

    let initial_under_node_id = under.map(|s| s.to_string());
    let mut last_node_id: Option<String> = None;
    let mut created_node_ids: Vec<String> = Vec::new();

    for (i, (blurb, content)) in parts.iter().enumerate() {
        println!("Creating node for part {}: {}", i + 1, blurb);
        let new_node_id = node::create_node(
            blurb,
            content,
            initial_under_node_id.as_deref(),
            last_node_id.as_deref(),
        )
        .map_err(|e| anyhow!("Failed to create node: {}", e))?;

        last_node_id = Some(new_node_id.to_string());
        created_node_ids.push(new_node_id.to_string());
    }

    println!(
        "Successfully imported {} parts from '{}'.",
        parts.len(),
        file
    );

    // Validation step
    let mut reconstructed_content = String::new();
    for node_id in &created_node_ids {
        let content = node::get_node_content(node_id)
            .map_err(|e| anyhow!("Failed to get content for node {}: {}", node_id, e))?;
        reconstructed_content.push_str(&content);
        reconstructed_content.push_str("\n\n"); // Add double newline as a separator, matching dummy LLM
    }

    // Remove the last two newlines if they exist
    if reconstructed_content.ends_with("\n\n") {
        reconstructed_content.truncate(reconstructed_content.len() - 2);
    }

    if reconstructed_content == original_content {
        println!("Validation successful: Reconstructed content matches original.");
    } else {
        eprintln!("Validation failed: Reconstructed content does NOT match original.");
        // For debugging, you might want to print diffs here
        // eprintln!("Original:\n{}", original_content);
        // eprintln!("Reconstructed:\n{}", reconstructed_content);
    }

    Ok(())
}
