use crate::config::{FullConfig, LlmSettings};
use crate::node::NodeManager;
use std::fs;

pub fn run(blurb: Option<&str>) {
    let starting_node_title = blurb.unwrap_or("Starting Node");
    let starting_node_id =
        NodeManager::create_node(starting_node_title, starting_node_title, None, None).unwrap();

    let bok_config = FullConfig {
        llm: LlmSettings::default(),
        title: Some("My New Book".to_string()),
        author: Some("Unknown Author".to_string()),
        starting_node: Some(starting_node_id.to_string()),
    };

    let yaml_content = serde_yaml::to_string(&bok_config).unwrap();

    fs::write("bok.yaml", yaml_content).unwrap();
    println!("Created bok.yaml and starting node directory.");
}
