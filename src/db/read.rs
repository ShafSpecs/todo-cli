use serde_json::{Value, from_str, json};
use std::fs::read_to_string;

pub fn read_db() -> Value {
  let mut db_contents = read_to_string("./db.json").unwrap();

  if db_contents == "" {
    db_contents = "{}".to_string();
  }

  let todo: Value = from_str(&db_contents).unwrap();

  return todo

  // let todo_list:
  //   serde_json::Value = serde_json::from_str(&todo["todo_list"].to_string()).unwrap();

}