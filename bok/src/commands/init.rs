use std::fs;
use std::io::Cursor;
use murmur3::murmur3_32;

pub fn run() {
    let starting_node_title = "Starting Node";
    let mut reader = Cursor::new(starting_node_title.as_bytes());
    let starting_node_id = murmur3_32(&mut reader, 0).unwrap();

    let hocon_content = format!(
        "book: {{
title: \"My New Book\",
author: \"Unknown Author\",
starting_node: \"{id} {title}\"
}}",
        id = starting_node_id,
        title = starting_node_title
    );

    fs::write("book.conf", hocon_content).unwrap();

    let node_dir_name = format!("{} {}", starting_node_id, starting_node_title);
    fs::create_dir_all(&node_dir_name).unwrap();
    println!("Created book.conf and starting node directory.");
}
