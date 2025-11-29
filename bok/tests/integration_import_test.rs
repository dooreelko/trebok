use assert_cmd::cargo::cargo_bin;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_import_command_with_dummy_provider() -> Result<(), Box<dyn std::error::Error>> {
    // Setup: Create a temporary directory for the test
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    let test_file_name = "test_import_file.md";
    let test_file_path = temp_path.join(test_file_name);
    let test_file_content =
        "This is the first part.\n\nThis is the second part.\n\n# Heading for third part.";
    fs::write(&test_file_path, test_file_content)?;

    // Create bok.yaml for the test with dummy provider
    let bok_yaml_content = r#"llm:
  provider: dummy
  model: "qwen3:8b"
  location: http://localhost
  port: 11434
"#;
    fs::write(temp_path.join("bok.yaml"), bok_yaml_content)?;

    // Run the import command from the temporary directory
    let mut cmd = Command::new(cargo_bin!("bok"));
    cmd.current_dir(&temp_path); // Set the current directory for the command
    cmd.arg("import").arg(test_file_name);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Using Dummy provider."))
        .stdout(predicates::str::contains(
            "Successfully imported 3 parts from 'test_import_file.md'.",
        ))
        .stdout(predicates::str::contains(
            "Validation successful: Reconstructed content matches original.",
        ));

    // Verify node creation
    let mut node_count = 0;
    for entry in fs::read_dir(&temp_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                if dir_name
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .parse::<u32>()
                    .is_ok()
                {
                    node_count += 1;
                    assert!(path.join("meta.yaml").exists());
                    assert!(path.join("text.qmd").exists());
                }
            }
        }
    }
    assert_eq!(node_count, 3, "Expected 3 nodes to be created.");

    // Cleanup: Remove the temporary directory
    // The TempDir will be automatically removed when it goes out of scope
    // fs::remove_dir_all(&temp_dir)?; // No need to explicitly remove

    Ok(())
}

#[cfg(feature = "ollama_tests")]
#[test]
fn test_import_command_with_ollama_provider() -> Result<(), Box<dyn std::error::Error>> {
    // Setup: Create a dummy markdown file
    let test_file_name = "test_import_file_ollama.md";
    let test_file_content =
        "This is the first part.\n\nThis is the second part.\n\n# Heading for third part.";
    fs::write(test_file_name, test_file_content)?;

    // Ensure bok.conf exists for the test
    let bok_conf_content = r#"
    llm {
        provider = "ollama"
        model = "qwen3:8b"
        location = "http://localhost"
        port = 11434
    }
    "#;
    fs::write("bok.conf", bok_conf_content)?;

    // Run the import command
    let mut cmd = Command::new(cargo_bin!("bok"));
    cmd.arg("import").arg(test_file_name);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(
            "Successfully imported 3 parts from 'test_import_file_ollama.md'.",
        ))
        .stdout(predicates::str::contains(
            "Validation successful: Reconstructed content matches original.",
        ));

    // Verify node creation (dummy check for now, more robust checks can be added)
    let current_dir = std::env::current_dir()?;
    let mut node_count = 0;
    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                if dir_name
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .parse::<u32>()
                    .is_ok()
                {
                    node_count += 1;
                    assert!(path.join("meta.yaml").exists());
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
                if dir_name
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .parse::<u32>()
                    .is_ok()
                {
                    fs::remove_dir_all(&path)?;
                }
            }
        }
    }

    Ok(())
}
