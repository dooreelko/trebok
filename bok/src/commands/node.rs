use crate::node::NodeManager;

pub fn add(blurb: &str, under: Option<&str>) {
    match NodeManager::create_node(blurb, blurb, under, None) {
        Ok(_) => println!("Created new node directory, text.qmd and meta.yaml."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn rm(node_hash: &str) {
    match NodeManager::remove_node(node_hash) {
        Ok(path) => println!("Removed node '{}'", path.display()),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn ls() {
    if let Err(e) = NodeManager::list_nodes() {
        eprintln!("Error: {}", e);
    }
}

pub fn get_all_nodes_flat() -> Vec<(String, String)> {
    NodeManager::get_all_nodes_flat()
}
