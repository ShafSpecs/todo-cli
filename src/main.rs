mod cmd;
mod structs;
mod db;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Command {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new todo
    Add { todo: String },

    /// List todos
    List
}

fn main() {
    let input = Command::parse();

    match &input.command {
        Commands::Add { todo } => {
            println!("Toto {:?}", todo)
        },
        Commands::List => {
            println!("All lists")
        }
    };
}
