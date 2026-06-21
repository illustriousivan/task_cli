mod app;
mod core;
mod storage;

use clap::Parser;
use app::App;
use core::commands::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    // Initialize app with storage
    let storage = Box::new(storage::json_storage::JsonStorage::new("tasks.json"));
    let mut app = App::new(storage);

    // Dispatch command through App
    match app.dispatch(cli.command) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
