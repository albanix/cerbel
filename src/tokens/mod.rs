/// Enum of tokens
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
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