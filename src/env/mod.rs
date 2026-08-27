use std::collections::HashMap;
use std::env;
use std::fs;

pub struct Environment {
    pub variables: HashMap<String, f64>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn set(&mut self, name: String, value: f64) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> f64 {
        match self.variables.get(name) {
            Some(v) => *v,
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