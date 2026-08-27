use crate::{env::Environment, expr::Expr, parser::expr::eval, tokens::value::Value};

#[derive(Debug, Clone)]
pub enum Stmt {
    Def(String, Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    ExprStmt(Expr)
}


pub fn exec(stmt: &Stmt, env: &mut Environment) {
    match stmt {
        Stmt::Def(name, expr) => {
            let value = eval(expr, env);
            env.set(name.clone(), value);
        }

        Stmt::ExprStmt(expr) => {
            let value = eval(expr, env);
            println!("=> {:?}", value);
        }

        Stmt::If { condition, then_branch, else_branch } => {
            let cond = eval(condition, env);
            let is_true = match cond {
                Value::Bool(b) => b,
                other => panic!("IF condition must be bool, got: {:?}", other),
            };

            if is_true {
                for stmt in then_branch {
                    exec(stmt, env);
                }
            } else if let Some(branch) = else_branch {
                for stmt in branch {
                    exec(stmt, env);
                }
            }
        }
    }
}