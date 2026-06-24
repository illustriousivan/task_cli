use clap::Subcommand;

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// Create a new task with description
    #[command(name = "create")]
    Create {
        #[arg(long)]
        description: Option<String>,
    },

    /// Remove a task by ID
    #[command(name = "remove")]
    Remove { id: u32 },

    /// Update an existing task
    #[command(name = "update")]
    Update {
        id: u32,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },

    /// List all tasks
    #[command(name = "list")]
    List {
        #[arg(long)]
        all: bool,

        #[arg(long)]
        status: Option<String>,
    },

    /// Clear all tasks or mark them as done
    #[command(name = "clear")]
    Clear {
        #[arg(long)]
        yes: bool,

        #[arg(long)]
        done: bool,
    },
}
