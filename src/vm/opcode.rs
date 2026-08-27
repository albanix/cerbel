use crate::tokens::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Push(Value),    // push value in stack
    Load(String),   // get variables from Environment, push in stack
    Store(String),  // Remove from stak, safe in Environmet

    Add, Sub, Mul, Div,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,

    JumpIfFalse(usize),     // if by stack false - go to index
    Jump(usize),    // Unconditional jump
    Pop,        // Pop the top of the stack (for ExprStmt after printing, or discarding a value)
    Print,      // Print the top of the stack
}

