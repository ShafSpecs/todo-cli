use crate::structs::todo;
use crate::db::write;

pub fn create_todo(todo_content: String) {
  let new_todo = todo::Todo::new(&todo_content);

  write::write_to_db(new_todo);
}