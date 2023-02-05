pub struct Todo {
    key: String,
    todo: String,
}

impl Todo {
    pub fn new(todo: &str) -> Todo {
        Todo { key: "key".to_string(), todo: String::from(todo) }
    }

    pub fn get_todo(self) -> String {
      self.todo
    }

    pub fn edit(key: String, todo: &str) -> Todo {
      Todo { key, todo: String::from(todo) }
    }

    pub fn delete(key: String, todo: &str) -> () {
      ()
    }
}
