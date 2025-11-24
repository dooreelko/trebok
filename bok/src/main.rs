use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells};

mod commands;

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
        /// A short blurb for the new node
        blurb: Vec<String>,
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
        Commands::Init { blurb } => {
            if blurb.is_empty() {
                commands::init::run(None)
            } else {
                commands::init::run(Some(&blurb.join(" ")))
            }
        }
        Commands::Node { action } => match action {
            NodeAction::Add { blurb } => commands::node::add(&blurb.join(" ")),
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
