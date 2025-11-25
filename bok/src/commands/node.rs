use glob::glob;
use hocon::HoconLoader;
use murmur3::murmur3_32;
use std::fs;
use std::io::Cursor;
use std::path::Path;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub blurb: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(id: String, blurb: String) -> Self {
        Node {
            id,
            blurb,
            children: Vec::new(),
        }
    }
}

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

pub fn rm(node_hash: &str) {
    let pattern = format!("./{} *", node_hash);
    let mut node_path_opt = None;
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        if let Ok(p) = entry {
            if p.is_dir() {
                node_path_opt = Some(p);
                break;
            }
        }
    }

    if let Some(node_path) = node_path_opt {
        match fs::remove_dir_all(&node_path) {
            Ok(_) => println!("Removed node '{}'", node_path.display()),
            Err(e) => println!("Error removing node '{}': {}", node_path.display(), e),
        }
    } else {
        println!("Error: Node with hash '{}' not found.", node_hash);
    }
}

fn print_nodes_recursive(nodes: &[Node], prefix: &str) {
    for node in nodes {
        println!("{}{}", prefix, node.blurb);
        print_nodes_recursive(&node.children, &format!("  {}", prefix));
    }
}

fn get_nodes_recursive(dir: &Path) -> Vec<Node> {
    let mut nodes = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        let mut sorted_entries: Vec<_> = entries.filter_map(Result::ok).collect();
        sorted_entries.sort_by_key(|entry| entry.path());

        for entry in sorted_entries {
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("meta.hocon");
                if meta_path.exists() {
                    let node_name = path.file_name().unwrap().to_str().unwrap();
                    let parts: Vec<&str> = node_name.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        let id = parts[0].to_string();
                        let blurb = parts[1].to_string();
                        let mut node = Node::new(id, blurb);
                        node.children = get_nodes_recursive(&path);
                        nodes.push(node);
                    }
                }
            }
        }
    }
    nodes
}

pub fn ls() {
    let hocon = HoconLoader::new()
        .load_file("book.conf")
        .expect("Unable to load book.conf")
        .hocon()
        .unwrap();
    let starting_node_id = hocon["book"]["starting_node"].as_string().unwrap();

    let mut all_nodes = get_nodes_recursive(Path::new("."));

    let mut starting_node_index = None;
    for (i, node) in all_nodes.iter().enumerate() {
        if node.id == starting_node_id {
            starting_node_index = Some(i);
            break;
        }
    }

    if let Some(index) = starting_node_index {
        let starting_node = all_nodes.remove(index);
        println!("{}", starting_node.blurb);
        print_nodes_recursive(&starting_node.children, "  ");
    } else {
        eprintln!("Starting node with id {} not found.", starting_node_id);
    }

    // Print remaining root nodes
    for node in all_nodes {
        println!("{}", node.blurb);
        print_nodes_recursive(&node.children, "  ");
    }
}

pub fn get_all_nodes_flat() -> Vec<(String, String)> {
    let mut nodes_flat = Vec::new();
    get_all_nodes_flat_recursive(Path::new("."), &mut nodes_flat);
    nodes_flat
}

fn get_all_nodes_flat_recursive(dir: &Path, nodes_flat: &mut Vec<(String, String)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let meta_path = path.join("meta.hocon");
                if meta_path.exists() {
                    let node_name = path.file_name().unwrap().to_str().unwrap();
                    let parts: Vec<&str> = node_name.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        let id = parts[0].to_string();
                        let blurb = parts[1].to_string();
                        nodes_flat.push((id, blurb));
                        get_all_nodes_flat_recursive(&path, nodes_flat);
                    }
                }
            }
        }
    }
}
