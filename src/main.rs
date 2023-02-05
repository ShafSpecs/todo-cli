mod structs;

use structs::todo;

fn main() {
    println!("Hello, world!");
}

fn create_new_todo() -> todo::Todo {
    let todo = todo::Todo::new("Hello World");

    todo
}
