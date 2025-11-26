use assert_cmd::cargo::cargo_bin;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::fs;

#[test]
fn test_import_command_with_ollama_provider() -> Result<(), Box<dyn std::error::Error>> {
    // Setup: Create a dummy markdown file
    let test_file_name = "test_import_file.md";
    let test_file_content = "This is the first part.\n\nThis is the second part.\n\n# Heading for third part.";
    fs::write(test_file_name, test_file_content)?;

    // Ensure bok.conf exists for the test
    let bok_conf_content = r#"
    llm {
        provider = "ollama"
        model = "qwen3:14b"
        location = "http://localhost:11434"
    }
    "#;
    fs::write("bok.conf", bok_conf_content)?;

    // Run the import command
    let mut cmd = Command::new(cargo_bin("bok"));
    cmd.arg("import").arg(test_file_name);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully imported 3 parts from 'test_import_file.md'."))
        .stdout(predicate::str::contains("Validation successful: Reconstructed content matches original."));

    // Verify node creation (dummy check for now, more robust checks can be added)
    // The dummy provider splits by double newline, so we expect 3 nodes.
    // We can't easily predict the node IDs, so we'll check for directories that look like nodes.
    let current_dir = std::env::current_dir()?;
    let mut node_count = 0;
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                // Node directories are named like "ID blurb"
                if dir_name.split_whitespace().next().unwrap_or("").parse::<u32>().is_ok() {
                    node_count += 1;
                    // Check for meta.hocon and text.qmd inside the node directory
                    assert!(path.join("meta.hocon").exists());
                    assert!(path.join("text.qmd").exists());
                }
            }
        }
    }
    assert_eq!(node_count, 3, "Expected 3 nodes to be created.");

    // Cleanup: Remove the dummy markdown file, bok.conf, and created node directories
    fs::remove_file(test_file_name)?;
    fs::remove_file("bok.conf")?;
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                if dir_name.split_whitespace().next().unwrap_or("").parse::<u32>().is_ok() {
                    fs::remove_dir_all(&path)?;
                }
            }
        }
    }

    Ok(())
}