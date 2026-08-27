use std::collections::HashMap;

use crate::{tokens::value::Value, vm::opcode::OpCode};

pub mod opcode;
pub mod compiler;

pub struct VM {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    pc: usize
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self, code: &[OpCode]) {
        while self.pc < code.len() {
            match &code[self.pc] {
                OpCode::Push(v) => self.stack.push(v.clone()),

                OpCode::Load(name) => {
                    let v = self.variables.get(name)
                        .unwrap()
                        .clone();
                    self.stack.push(v);
                }

                OpCode::Store(name) => {
                    let v = self.stack.pop().unwrap();
                    self.variables.insert(name.clone(), v);
                }

                OpCode::Add => self.binary_op(|a,b| add_values(a, b)),
                OpCode::Sub => self.binary_op(|a, b| numeric_op(a, b, |x, y| x - y)),
                OpCode::Mul => self.binary_op(|a, b| numeric_op(a, b, |x, y| x * y)),
                OpCode::Div => self.binary_op(|a, b| numeric_op(a, b, |x, y| x / y)),

                OpCode::Eq => self.binary_op(|a, b| Value::Bool(a == b)),
                OpCode::NotEq => self.binary_op(|a, b| Value::Bool(a != b)),
                OpCode::Lt => self.binary_op(|a, b| compare_op(a, b, |x, y| x < y)),
                OpCode::Gt => self.binary_op(|a, b| compare_op(a, b, |x, y| x > y)),
                OpCode::GtEq => self.binary_op(|a, b| compare_op(a, b, |x, y| x >= y)),
                OpCode::LtEq => self.binary_op(|a, b| compare_op(a, b, |x, y| x <= y)),

                OpCode::Print => {
                    let v = self.stack.pop().unwrap();
                    println!("=> {:?}", v);
                }

                OpCode::Pop => { self.stack.pop(); },

                OpCode::Jump(target) => {
                    self.pc = *target;
                    continue;
                }

                OpCode::JumpIfFalse(target) => {
                    let v = self.stack.pop().unwrap();
                    let is_false = matches!(v, Value::Bool(false));
                    if is_false {
                        self.pc = *target;
                        continue;
                    }
                }

            }

            self.pc += 1;
        }
    }

    fn binary_op(&mut self, f: impl Fn(Value, Value) -> Value) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(f(a, b));
    }
}



fn numeric_op(a: Value, b: Value, f: impl Fn(f64, f64) -> f64) -> Value {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Value::Number(f(x, y)),
        (a, b) => panic!("Type mismatch: {:?} and {:?}", a, b),
    }
}

fn compare_op(a: Value, b: Value, f: impl Fn(f64, f64) -> bool) -> Value {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Value::Bool(f(x, y)),
        (a, b) => panic!("Type mismatch: {:?} and {:?}", a, b)
    }
}

fn add_values(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
        (Value::Str(x), Value::Str(y)) => Value::Str(x + &y),
        (a, b) => panic!("Type mismatch: {:?} and {:?}", a, b)
    }
}