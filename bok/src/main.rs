use clap::{builder::PossibleValuesParser, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells};

mod commands;

// HACK: This is a workaround for clap's dynamic completions.
// It leaks memory, but this function is only called when generating completions,
// so it's a small, one-time leak.
fn get_node_hashes_for_clap() -> Vec<&'static str> {
    let nodes = commands::node::get_all_nodes_flat();
    let formatted_nodes: Vec<String> = nodes.into_iter().map(|(id, blurb)| format!("{}-{}", id, blurb.replace(' ', "-"))).collect();
    let leaked: &'static Vec<String> = Box::leak(Box::new(formatted_nodes));
    leaked.iter().map(|s| s.as_str()).collect()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new book
    Init {
        /// An optional blurb for the starting node
        blurb: Vec<String>,
    },
    /// Adds, removes, or lists nodes
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },
    /// Visualizes the book
    Vis {
        #[command(subcommand)]
        format: VisFormat,
    },
    /// Generates the book in a specific format
    Generate {
        #[command(subcommand)]
        format: GenerateFormat,
    },
    /// Line edits a node
    Lineedit {
        /// The node to line edit
        node: String,
    },
    /// Copy edits a node
    Copyedit {
        /// The node to copy edit
        node: String,
    },
    /// Runs checks on the book
    Check,
    /// Imports a qmd file
    Import {
        /// The qmd file to import
        file: String,
    },
    /// Generate shell completions
    Completion {
        #[arg(value_enum)]
        shell: shells::Shell,
    },
}

#[derive(Subcommand)]
enum NodeAction {
    /// Adds a new node
    Add {
        /// The parent node hash under which to create the new node
        #[arg(long, value_parser = PossibleValuesParser::new(get_node_hashes_for_clap()))]
        under: Option<String>,
        /// A short blurb for the new node
        blurb: Vec<String>,
    },
    /// Removes a node
    Rm {
        /// The node to remove
        #[arg(value_parser = PossibleValuesParser::new(get_node_hashes_for_clap()))]
        node: String,
    },
    /// Lists the node hierarchy
    Ls,
}

#[derive(Subcommand)]
enum VisFormat {
    /// Generates a d3 json file
    D3,
    /// Generates a mermaid diagram
    Mermaid,
}

#[derive(Subcommand)]
enum GenerateFormat {
    /// Generates a quarto book
    Quarto,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { blurb } => {
            if blurb.is_empty() {
                commands::init::run(None)
            } else {
                commands::init::run(Some(&blurb.join(" ")))
            }
        }
        Commands::Node { action } => match action {
            NodeAction::Add { blurb, under } => {
                let parsed_under = under.as_ref().map(|s| s.splitn(2, '-').next().unwrap_or("").to_string());
                commands::node::add(&blurb.join(" "), parsed_under.as_deref())
            }
            NodeAction::Rm { node } => {
                let parsed_node = node.splitn(2, '-').next().unwrap_or("").to_string();
                commands::node::rm(&parsed_node)
            },
            NodeAction::Ls => commands::node::ls(),
        },
        Commands::Vis { format } => match format {
            VisFormat::D3 => commands::vis::d3(),
            VisFormat::Mermaid => commands::vis::mermaid(),
        },
        Commands::Generate { format } => match format {
            GenerateFormat::Quarto => commands::generate::quarto(),
        },
        Commands::Lineedit { node } => commands::lineedit::run(node),
        Commands::Copyedit { node } => commands::copyedit::run(node),
        Commands::Check => commands::check::run(),
        Commands::Import { file } => commands::import::run(file),
        Commands::Completion { shell } => {
            generate(
                *shell,
                &mut Cli::command(),
                "bok",
                &mut std::io::stdout(),
            );
        }
    }
}

