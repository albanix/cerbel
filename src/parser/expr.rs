use crate::{env::Environment, tokens::Token};
/// The Expr enum will store itself recursively, to represent operator precedence.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    }
}

pub fn eval(expr: &Expr, env: &Environment) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Identifier(name) => env.get(name),
        Expr::BinaryOp { left, op, right } => {
            let l = eval(left, env);
            let r = eval(right, env);
            match op {
                Token::Plus => l + r,
                Token::Minus => l - r,
                Token::Star => l * r,
                Token::Slash => l / r,
                Token::EqualEqual => if l == r { 1.0 } else { 0.0 },
                Token::NotEqual => if l != r { 1.0 } else { 0.0 },
                Token::LessEqual => if l <= r { 1.0 } else { 0.0 },
                Token::GreaterEqual => if l >= r { 1.0 } else { 0.0 },
                Token::Less => if l < r { 1.0 } else { 0.0 },
                Token::Greater => if l > r { 1.0 } else { 0.0 },
                _ => panic!("Unknown operator {:?}", op)
            }
        }
    }
}