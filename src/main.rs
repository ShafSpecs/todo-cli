mod cmd;
mod structs;
mod db;

use clap::{Parser, Subcommand};
use cmd::create;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Command {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new todo
    Add { todo: Vec<String> },

    /// List todos
    List
}

fn main() {
    let input = Command::parse();

    match &input.command {
        Commands::Add { todo } => {
            create::create_todo(todo.join(" "))
        },
        Commands::List => {
            println!("All lists")
        }
    };
}
