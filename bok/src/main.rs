use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new book
    Init,
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
}

#[derive(Subcommand)]
enum NodeAction {
    /// Adds a new node
    Add {
        /// A short blurb for the new node
        blurb: String,
    },
    /// Removes a node
    Rm {
        /// The node to remove
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
        Commands::Init => commands::init::run(),
        Commands::Node { action } => match action {
            NodeAction::Add { blurb } => commands::node::add(blurb),
            NodeAction::Rm { node } => commands::node::rm(node),
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
    }
}