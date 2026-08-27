use crate::{env::Environment, tokens::{Token, value::Value}};
/// The Expr enum will store itself recursively, to represent operator precedence.
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    }
}

pub fn eval(expr: &Expr, env: &Environment) -> Value {
    match expr {
        Expr::Literal(v) => v.clone(),
        Expr::Identifier(name) => env.get(name),
        Expr::BinaryOp { left, op, right } => {
            let l = eval(left, env);
            let r = eval(right, env);
            match (l, r) {
                (Value::Number(a), Value::Number(b)) => match op {
                    Token::Plus => Value::Number(a + b),
                    Token::Minus => Value::Number(a - b),
                    Token::Star => Value::Number(a * b),
                    Token::Slash => Value::Number(a / b),
                    Token::EqualEqual => Value::Bool(a == b),
                    Token::NotEqual => Value::Bool(a != b),
                    Token::LessEqual => Value::Bool(a <= b),
                    Token::GreaterEqual => Value::Bool(a >= b),
                    Token::Less => Value::Bool(a < b),
                    Token::Greater => Value::Bool(a > b),
                    _ => panic!("Unknown integer operator: {:?}", op),
                },
                (Value::Str(a), Value::Str(b)) => match op {
                    Token::Plus => Value::Str(a + &b),
                    Token::EqualEqual => Value::Bool(a == b),
                    Token::NotEqual => Value::Bool(a != b),
                    _ => panic!("Unknown string operator: {:?}", op),
                },
                (a, b) => panic!("Type mismatch: {:?}:{:?}", a, b)
            }
        }
    }
}