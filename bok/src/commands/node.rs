use std::fs;
use std::io::Cursor;
use murmur3::murmur3_32;
use hocon::HoconLoader;
use glob::glob;

pub fn add(blurb: &str) {
    let mut reader = Cursor::new(blurb.as_bytes());
    let node_id = murmur3_32(&mut reader, 0).unwrap();
    let node_dir_name = format!("{} {}", node_id, blurb);
    fs::create_dir_all(&node_dir_name).unwrap();

    let text_file_path = format!("{}/text.qmd", node_dir_name);
    fs::write(text_file_path, "").unwrap();

    let meta_file_path = format!("{}/meta.hocon", node_dir_name);
    let meta_content = format!(r#"title: "{}""#, blurb);
    fs::write(meta_file_path, meta_content).unwrap();
    println!("Created new node directory, text.qmd and meta.hocon.");
}

pub fn rm(node: &str) {
    match fs::remove_dir_all(node) {
        Ok(_) => println!("Removed node '{}'", node),
        Err(e) => println!("Error removing node '{}': {}", node, e),
    }
}

pub fn ls() {
    let hocon = HoconLoader::new()
        .load_file("book.conf")
        .expect("Unable to load book.conf")
        .hocon()
        .unwrap();
    let starting_node = hocon["book"]["starting_node"].as_string().unwrap();
    
    println!("{}", starting_node);
    
    // For now, we just list all nodes at the root level.
    // In the future, this will recursively traverse the hierarchy.
    for entry in glob("./*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    let meta_path = path.join("meta.hocon");
                    if meta_path.exists() {
                        let hocon = HoconLoader::new()
                            .load_file(meta_path)
                            .expect("Unable to load hocon file")
                            .hocon()
                            .unwrap();
                        let title = hocon["title"].as_string().unwrap_or_default();
                        if path.to_str().unwrap() != starting_node {
                            println!("  {}", title);
                        }
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
