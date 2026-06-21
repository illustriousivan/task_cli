use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// Create a new task with description
    #[command(name = "create")]
    Create {
        description: String,
    },

    /// Remove a task by ID
    #[command(name = "remove")]
    Remove {
        id: u32,
    },

    /// Update an existing task's description
    #[command(name = "update")]
    Update {
        id: u32,
        description: String,
    },

    /// List all tasks
    #[command(name = "list")]
    List,
}
