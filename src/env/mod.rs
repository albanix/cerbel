use std::collections::HashMap;
use std::env;
use std::fs;

use crate::tokens::value::Value;

pub struct Environment {
    pub variables: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Value {
        match self.variables.get(name) {
            Some(v) => v.clone(),
            None => panic!("Variable are not initialized: {}", name)
        }
    }
}

pub fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Reading.. {file_path}");

    let contents = fs::read_to_string(file_path).expect("IDK");

    contents
}