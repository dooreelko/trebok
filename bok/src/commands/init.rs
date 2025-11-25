use super::node;
use std::fs;
use hocon::HoconLoader;

pub fn run(blurb: Option<&str>) {
    let starting_node_title = blurb.unwrap_or("Starting Node");
    let starting_node_id = node::create_node(starting_node_title, None, None).unwrap();

    let hocon_content_string = format!(
        r#"
        book {{
            title = "My New Book"
            author = "Unknown Author"
            starting_node = "{}"
        }}
        "#,
        starting_node_id
    );

    // Validate the HOCON string before writing
    HoconLoader::new()
        .load_str(&hocon_content_string)
        .unwrap()
        .hocon()
        .unwrap();

    fs::write("book.conf", hocon_content_string).unwrap();
    println!("Created book.conf and starting node directory.");
}
