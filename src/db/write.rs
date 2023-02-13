use serde_json::{Value, json};
use crate::{structs::{todo::Todo, collection::Collection}, db::read};
use std::fs::write;

pub fn write_to_db(todo: Todo, category: Option<String>) -> () {
    let current_db_content = read::read_db();

    if current_db_content.to_string() == "{}" {
        write("./db.json", init_db().to_string()).unwrap();
    }

    if category.unwrap_or(String::from("Uncategorized")) == "Uncategorized" {

        // write("./db.json", new_db_content.to_string()).unwrap();
    }


    // write("./db.json", Todo::get_todo(todo).as_bytes()).unwrap();
    ()
}

fn init_db() -> Value {
    return json!({
        "name": "todo-cli",
        "categories": [
            {
                "key": "1",
                "name": "Uncategorized",
                "todos": []
            }
        ]
    })
}
