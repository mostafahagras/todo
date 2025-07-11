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

    /// Deletes the todo file for the current directory.
    /// Use todo help remove for other options
    #[command(alias = "d")]
    Delete(DeleteArgs),

    /// Update todo cli
    #[command()]
    Update,

    /// Fuzzily find todos
    #[command(alias = "find")]
    Search { query: Option<String> },

    /// Fuzzily find todos, check them
    #[command(alias = "done")]
    Check {
        query: Option<String>,
        /// Check all todos
        #[arg(short, long)]
        all: bool,
    },

    /// Fuzzily find todos, uncheck them
    #[command(alias = "undo")]
    Uncheck {
        query: Option<String>,
        /// Uncheck all todos
        #[arg(short, long)]
        all: bool,
    },

    /// Fuzzily find todos, remove them
    #[command(alias = "rm")]
    Remove {
        query: Option<String>,
        /// Remove all todos without deleting the file
        #[arg(short, long)]
        all: bool,
    },
}

#[derive(Debug, Parser)]
#[command(group(
    ArgGroup::new("mode")
        .required(false)
        .args(["all", "recurse"])
))]
pub struct DeleteArgs {
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
