use clap::{ArgGroup, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "todo",
    version,
    about = "An easy way to manage your todos",
    long_about = None,
    subcommand_required=false,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a hard link to this directory's todo file in the current directory
    #[command(alias = "s")]
    Sync,

    /// Undo the sync command
    #[command()]
    Unsync,

    /// List the todos for the current directory
    #[command(alias = "l")]
    List,

    /// Interactive configuration for the todo cli
    Config(ConfigArgs),

    /// Removes the todo for the current directory.
    /// Use todo help remove for other options
    #[command(alias = "rm")]
    Remove(RemoveArgs),

    /// Update todo cli
    #[command()]
    Update,

    /// Fuzzily find todos
    #[command(alias = "find")]
    Search { query: String },

    /// Fuzzily find todos, check them
    #[command(alias = "done")]
    Check { query: String },

    /// Fuzzily find todos, uncheck them
    #[command(alias = "undo")]
    Uncheck { query: String },
}

#[derive(Debug, Parser)]
#[command(group(
    ArgGroup::new("mode")
        .required(false)
        .args(["all", "recurse"])
))]
pub struct RemoveArgs {
    /// Remove all todos.
    #[arg(short, long, group = "mode")]
    pub all: bool,

    /// Remove in the current directory recursively
    #[arg(short, long, group = "mode")]
    pub recurse: bool,
}

#[derive(Debug, Parser)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: Option<ConfigSubcommand>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum ConfigSubcommand {
    /// Show the current config
    #[command(alias = "show")]
    List,
}
