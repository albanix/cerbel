pub mod lexer;
pub mod tokens;
pub mod parser;
pub mod env;
pub mod vm;

use crate::env::Environment;
use crate::lexer::Lexer;
use crate::parser::stmt::exec;
use crate::parser::{Parser, expr};
use crate::vm::VM;
use crate::vm::compiler::compile_stmt;
use crate::vm::opcode::OpCode;
fn main() {
    let input = env::read_file();
    // Заправляем лексер токенами
    let mut lexer = Lexer::new(&input);
    
    // Получаем все токены примерно [Number(2), Plus, LParen, Number(2), Star, Number(2), RParen]
    let token = lexer.tokenize();
    // инитим парсер
    let mut parser = Parser::new(token);
    let program = parser.parse_program();
    let mut code: Vec<OpCode> = Vec::new();
    for stmt in &program {
        compile_stmt(stmt, &mut code);
    }

    let mut vm = VM::new();
    vm.run(&code);
}