use std::fs;
use super::node;

pub fn run(blurb: Option<&str>) {
    let starting_node_title = blurb.unwrap_or("Starting Node");
    let starting_node_id = node::create_node(starting_node_title, None).unwrap();

    let hocon_content = format!(
        "book: {{
title: \"My New Book\",
author: \"Unknown Author\",
starting_node: \"{}\"
}}",
        starting_node_id
    );

    fs::write("book.conf", hocon_content).unwrap();
    println!("Created book.conf and starting node directory.");
}
