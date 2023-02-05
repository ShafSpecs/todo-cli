use std::{fmt::Error, fs::{File, write}, io::Write};
use crate::structs::todo::Todo;

pub fn write_to_db(todo: Todo) -> () {
    // let db_file = File::open("./db.json");
    // let mut db_file = match db_file {
    //     Ok(file) => file,
    //     Err(_error) => create_db_file().unwrap(),
    // };

    // db_file.write(.as_bytes()).unwrap();
    write("./db.json", Todo::get_todo(todo))
}

fn create_db_file() -> Result<File, Error> {
    let db_file = File::create("./db.json");
    let db_file = match db_file {
        Ok(file) => Ok(file),
        Err(error) => panic!("Error creating file: {}", error)
    };

    db_file
}
