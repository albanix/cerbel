use crate::{parser::{expr::Expr, stmt::Stmt}, tokens::Token, vm::opcode::OpCode};

pub fn compiler_expr(expr: &Expr, code: &mut Vec<OpCode>) {
    match expr {
        Expr::Literal(v) => code.push(OpCode::Push(v.clone())),
        Expr::Identifier(name) => code.push(OpCode::Load(name.clone())),
        Expr::BinaryOp { left, op, right } => {
            compiler_expr(left, code);
            compiler_expr(right, code);

            let opcode = match op {
                Token::Plus => OpCode::Add,
                Token::Minus => OpCode::Sub,
                Token::Star => OpCode::Mul,
                Token::Slash => OpCode::Div,
                Token::EqualEqual => OpCode::Eq,
                Token::NotEqual => OpCode::NotEq,
                Token::Less => OpCode::Lt,
                Token::Greater => OpCode::Gt,
                Token::GreaterEqual => OpCode::GtEq,
                Token::LessEqual => OpCode::LtEq,
                _ => panic!("Unknown operator: {:?}", op),
            };

            code.push(opcode);
        }
    }
}


pub fn compile_stmt(stmt: &Stmt, code: &mut Vec<OpCode>) {
    match stmt {
        Stmt::Def(name, expr) => {
            compiler_expr(expr, code);
            code.push(OpCode::Store(name.clone()));
        }

        Stmt::ExprStmt(expr) => {
            compiler_expr(expr, code);
            code.push(OpCode::Print);
        }

        Stmt::If { condition, then_branch, else_branch} => {
            compiler_expr(condition, code);
            let jump_if_false_pos = code.len();
            code.push(OpCode::JumpIfFalse(0));

            for s in then_branch {
                compile_stmt(s, code);
            }

            let jump_over_else_pos = code.len();
            code.push(OpCode::Jump(0));

            let else_start = code.len();
            code[jump_if_false_pos] = OpCode::JumpIfFalse(else_start);

            if let Some(branch) = else_branch {
                for s in branch {
                    compile_stmt(s, code);
                }
            }

            let after_if = code.len();
            code[jump_over_else_pos] = OpCode::Jump(after_if);
        }
    }
}