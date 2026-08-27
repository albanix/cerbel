pub mod lexer;
pub mod tokens;
pub mod parser;
pub mod env;

use crate::env::Environment;
use crate::lexer::Lexer;
use crate::parser::stmt::exec;
use crate::parser::{Parser, expr};
fn main() {
    let input = env::read_file();
    // Заправляем лексер токенами
    let mut lexer = Lexer::new(&input);
    
    // Получаем все токены примерно [Number(2), Plus, LParen, Number(2), Star, Number(2), RParen]
    let token = lexer.tokenize();
     
    println!("{:?}", token);
    // инитим парсер
    let mut parser = Parser::new(token);
    let program = parser.parse_program();
    let mut env = Environment::new();
    println!("{:#?}", program);

    for stmt in &program {
        exec(stmt, &mut env);
    }
}