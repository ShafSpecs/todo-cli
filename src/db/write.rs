use crate::structs::todo::Todo;
use std::fs::write;

pub fn write_to_db(todo: Todo) -> () {
    write("./db.json", Todo::get_todo(todo).as_bytes()).unwrap();
    ()
}
