use crate::node::NodeManager;
use hocon::HoconLoader;
use std::fs;

pub fn run(blurb: Option<&str>) {
    let starting_node_title = blurb.unwrap_or("Starting Node");
    let starting_node_id =
        NodeManager::create_node(starting_node_title, starting_node_title, None, None).unwrap();

    let hocon_content_string = format!(
        r#"
title = "My New Book"
author = "Unknown Author"
starting_node = "{}"
        "#,
        starting_node_id
    );

    // Validate the HOCON string before writing
    HoconLoader::new()
        .load_str(&hocon_content_string)
        .unwrap()
        .hocon()
        .unwrap();

    fs::write("bok.hocon", hocon_content_string).unwrap();
    println!("Created bok.hocon and starting node directory.");
}
