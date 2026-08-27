pub mod value;

/// Enum of tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    StringLiteral(String),
    True, False,
    Def,
    Equal, EqualEqual, NotEqual,
    Less, Greater,
    LessEqual, GreaterEqual,
    Semicolon,

    If,
    Else,

    Star, Slash, Plus, Minus,
    RParen, LParen, LBrace, RBrace,
    Eof
}