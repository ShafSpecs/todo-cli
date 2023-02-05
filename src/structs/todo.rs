pub struct Todo {
    key: String,
    todo: String,
}

impl Todo {
    pub fn new(todo: &str) -> Todo {
        Todo { key: "".to_string(), todo: String::from(todo) }
    }

    pub fn get(key: String) -> Todo {
      Todo { key, todo: String::new() }
    }

    pub fn edit(key: String, todo: &str) -> Todo {
      Todo { key, todo: String::from(todo) }
    }

    pub fn delete(key: String, todo: &str) -> () {
      ()
    }
}
