use crate::node::NodeManager;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct VisNode {
    id: String,
    title: String,
}

pub fn d3() {
    let nodes = NodeManager::get_nodes_recursive(Path::new("."));

    fn flatten_nodes(nodes: &[crate::node::Node], result: &mut Vec<VisNode>) {
        for node in nodes {
            result.push(VisNode {
                id: node.id.clone(),
                title: node.blurb().to_string(),
            });
            flatten_nodes(&node.children, result);
        }
    }

    let mut vis_nodes = Vec::new();
    flatten_nodes(&nodes, &mut vis_nodes);

    let json = serde_json::to_string_pretty(&vis_nodes).unwrap();
    println!("{}", json);
}

pub fn mermaid() {
    println!("'bok vis mermaid' is not yet implemented");
}
