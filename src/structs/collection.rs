use super::todo::Todo;

pub struct Collection {
  id: String,
  name: String,
  todos: Vec<Todo>
}

impl Collection {
  pub fn new(name: String) -> Collection {
    Collection {
      id: String::from(""),
      name,
      todos: Vec::new()
    }
  }

  pub fn get_id(&self) -> String {
    self.id.clone()
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_todos(self) -> Vec<Todo> {
    self.todos
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn set_todos(&mut self, todos: Vec<Todo>) {
    self.todos = todos;
  }
}