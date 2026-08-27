use crate::{env::Environment, expr::Expr, parser::expr::eval};

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
            println!("=> {}", value);
        }
    }
}