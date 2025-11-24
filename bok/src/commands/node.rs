use std::fs;
use std::io::Cursor;
use std::path::Path;
use murmur3::murmur3_32;
use hocon::HoconLoader;
use glob::glob;

pub fn create_node(blurb: &str, under: Option<&str>) -> Result<u32, String> {
    let mut reader = Cursor::new(blurb.as_bytes());
    let node_id = murmur3_32(&mut reader, 0).unwrap();
    let node_dir_name = format!("{} {}", node_id, blurb);

    let mut path = std::path::PathBuf::new();
    if let Some(under_hash) = under {
        let parent_pattern = format!("./{} *", under_hash);
        let mut parent_path_opt = None;
        for entry in glob(&parent_pattern).expect("Failed to read glob pattern") {
            if let Ok(p) = entry {
                if p.is_dir() {
                    parent_path_opt = Some(p);
                    break;
                }
            }
        }
        if let Some(parent_path) = parent_path_opt {
            path.push(parent_path);
        } else {
            return Err(format!("Parent node with hash {} not found.", under_hash));
        }
    }
    path.push(node_dir_name);

    fs::create_dir_all(&path).unwrap();

    let text_file_path = path.join("text.qmd");
    let qmd_content = format!("# {}", blurb);
    fs::write(text_file_path, qmd_content).unwrap();

    let meta_file_path = path.join("meta.hocon");
    let meta_content = format!(r#"title: "{}""#, blurb);
    fs::write(meta_file_path, meta_content).unwrap();
    
    Ok(node_id)
}

pub fn add(blurb: &str, under: Option<&str>) {
    match create_node(blurb, under) {
        Ok(_) => println!("Created new node directory, text.qmd and meta.hocon."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn rm(node: &str) {
    match fs::remove_dir_all(node) {
        Ok(_) => println!("Removed node '{}'", node),
        Err(e) => println!("Error removing node '{}': {}", node, e),
    }
}

fn ls_recursive(dir: &Path, prefix: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("meta.hocon");
                if meta_path.exists() {
                    let node_name = path.file_name().unwrap().to_str().unwrap();
                    println!("{}{}", prefix, node_name);
                    ls_recursive(&path, &format!("  {}", prefix));
                }
            }
        }
    }
}

pub fn ls() {
    let hocon = HoconLoader::new()
        .load_file("book.conf")
        .expect("Unable to load book.conf")
        .hocon()
        .unwrap();
    let starting_node_id = hocon["book"]["starting_node"].as_string().unwrap();

    let mut root_nodes = Vec::new();
    let mut starting_node_path_opt = None;

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("meta.hocon");
                if meta_path.exists() {
                    let node_name = path.file_name().unwrap().to_str().unwrap();
                    if node_name.starts_with(&starting_node_id) {
                        starting_node_path_opt = Some(path.clone());
                    } else {
                        root_nodes.push(path);
                    }
                }
            }
        }
    }

    if let Some(starting_node_path) = starting_node_path_opt {
        let node_name = starting_node_path.file_name().unwrap().to_str().unwrap();
        println!("{}", node_name);
        ls_recursive(&starting_node_path, "  ");
    } else {
        println!("Starting node with id {} not found.", starting_node_id);
    }

    root_nodes.sort();
    for path in root_nodes {
        let node_name = path.file_name().unwrap().to_str().unwrap();
        println!("{}", node_name);
        ls_recursive(&path, "  ");
    }
}

fn get_all_hashes_recursive(dir: &Path, hashes: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("meta.hocon");
                if meta_path.exists() {
                    let node_name = path.file_name().unwrap().to_str().unwrap();
                    if let Some(hash) = node_name.split_whitespace().next() {
                        hashes.push(hash.to_string());
                        get_all_hashes_recursive(&path, hashes);
                    }
                }
            }
        }
    }
}

pub fn get_all_node_hashes() -> Vec<String> {
    let mut hashes = Vec::new();
    get_all_hashes_recursive(Path::new("."), &mut hashes);
    hashes
}
