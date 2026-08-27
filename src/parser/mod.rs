use crate::{parser::{expr::Expr, stmt::Stmt}, tokens::Token};
pub mod expr;
pub mod stmt;
/// The parser is the component that builds the AST. It stores the current token (recall the lexer) and an index/pointer into the token stream. The parser defines operator precedence, among other things.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statemnts = Vec::new();

        while *self.current_token() != Token::Eof {
            statemnts.push(self.parse_statement());
        }

        statemnts
    }

    pub fn parse_statement(&mut self) -> Stmt {
        match self.current_token() {
            Token::Def => self.parse_def(),
            _ => self.parse_expr_statement()
        }
    }

    pub fn parse_def(&mut self) -> Stmt {
        self.advance();
        let name = match self.advance() {
            Token::Identifier(n) => n,
            other => panic!("Expected a variable name, got: {:?}", other)
        };

        match self.advance() {
            Token::Equal => {}
            other => panic!("Expected a '=', got: {:?}", other)
        }

        let value = self.parse_compression();

        match self.advance() {
            Token::Semicolon => {},
            other => panic!("Expected a ';', got: {:?}", other)
        }

        Stmt::Def(name, value)
    }


    pub fn parse_expr_statement(&mut self) -> Stmt {
        let expr = self.parse_compression();

        match self.advance() {
            Token::Semicolon => {},
            other => panic!("Expected ';', got: {:?}", other)
        }

        Stmt::ExprStmt(expr)
    }

    pub fn parse_compression(&mut self) -> Expr {
        let mut left = self.parse_expr();

        while matches!(self.current_token(), Token::EqualEqual | Token::NotEqual | Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual) {
            let op = self.advance();
            let right = self.parse_expr();
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        left
    }
    pub fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = self.advance();
            let right = self.parse_term();
            left = Expr::BinaryOp { 
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        left
    }

    pub fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while matches!(self.current_token(), Token::Star | Token::Slash) {
            let op = self.advance();
            let right = self.parse_factor();
            left = Expr::BinaryOp { 
                left: Box::new(left), 
                op, 
                right: Box::new(right)
            };
        }

        left
    }
    pub fn parse_factor(&mut self) -> Expr {
        match self.advance() {
            Token::Number(n) => Expr::Number(n),
            Token::Identifier(name) => Expr::Identifier(name),
            Token::LParen => {
                let expr = self.parse_expr();
                self.advance();
                expr
            }
            other => panic!("Unexpected token: {:?}", other),
        }
    }
}