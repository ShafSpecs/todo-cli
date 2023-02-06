use super::todo::Todo;

pub struct Collection {
  id: String,
  name: String,
  todos: Vec<Todo>
}