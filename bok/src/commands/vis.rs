use glob::glob;
use hocon::HoconLoader;
use serde::Serialize;

#[derive(Serialize)]
struct Node {
    id: String,
    title: String,
}

pub fn d3() {
    let mut nodes = Vec::new();
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
                        let id = path.file_name().unwrap().to_str().unwrap().to_string();
                        nodes.push(Node { id, title });
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    let json = serde_json::to_string_pretty(&nodes).unwrap();
    println!("{}", json);
}

pub fn mermaid() {
    println!("'bok vis mermaid' is not yet implemented");
}
