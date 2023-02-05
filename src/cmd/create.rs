use crate::structs::todo;

pub fn add_todo(todo_content: String) {
  let new_todo = todo::Todo::new(&todo_content);
}