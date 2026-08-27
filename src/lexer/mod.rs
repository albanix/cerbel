use crate::tokens::Token;

/// Lexer
/// Using a lexer, we turn a stream of tokens into more meaningful structures, for example, ( into RParen
///
/// 
/// 
/// char - contains a vector of characters
///
/// pos - a usize index that holds a pointer to the character
pub struct Lexer {
    char: Vec<char>,
    pos: usize
}

impl Lexer {
    /// We fill the `char` field. How? We take our input, then split it into an iterator and collect it into a vector of characters.
    /// 
    /// We set the index to 0; the step is done through `&mut self.advance()`.
    /// 
    /// ### Examples
    /// 
    /// ```
    /// // Yes... don't forget to import this (Lexer)
    /// let mut lexer = Lexer::new("your tokens...")
    /// ```
    pub fn new(input: &str) -> Self {
        Self {
            char: input.chars().collect(),
            pos: 0
        }
    }

    /// Tokenization: this is the process in which we get the current character via `self.current_char() -> Option<char>`. We use a while let Some construct and look for matches to give meaning to each character, because `(` or `*` or even `123` mean nothing to our computer on their own.
    /// 
    /// So tokenization will return you a Vector of tokens for convenient processing — Vec<Token>. I hope I don't have to explain how this process works? Right?.. Thanks.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' | '\n' => self.advance(),
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }

                ch if ch.is_ascii_digit() || ch == '.' => {
                    let n = self.read_number();
                    tokens.push(Token::Number(n));
                }

                ch if ch.is_ascii_alphabetic() || ch == '.' => {
                    let identifier = self.read_identifier();
                    let token = match identifier.as_str() {
                        "def" => Token::Def,
                        "if" => Token::If,
                        "else" => Token::Else,
                        _ => Token::Identifier(identifier)
                    };

                    tokens.push(token);
                }

                '=' => {
                    if self.peek_char() == Some('=') {
                        tokens.push(Token::EqualEqual);
                        self.advance();
                        self.advance();
                    } else {
                        tokens.push(Token::Equal);
                    }
                }

                '<' => {
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::LessEqual);
                    } else {
                        self.advance();
                        tokens.push(Token::Less);
                    }
                }

                '>' => {
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::GreaterEqual);
                    } else {
                        self.advance();
                        tokens.push(Token::Greater);
                    }
                }

                '!' => {
                    if self.peek_char() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::NotEqual);
                    } else {
                        panic!("! not supported!");
                    }
                }

                '{' => {
                    tokens.push(Token::LBrace);
                    self.advance();
                }

                '}' => {
                    tokens.push(Token::RBrace);
                    self.advance();
                }

                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }

                
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }

                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }

                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }

                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }

                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }

                _ => panic!("Unknown char {ch}"),
            }
        }

        tokens.push(Token::Eof);
        tokens
    }

    /// Here we take the vector and access the element in a safer way, using `.copied()` to remove the reference and get our symbol.
    pub fn current_char(&self) -> Option<char> {
        self.char.get(self.pos).copied()
    }

    /// We js increment the 'pointer' by one.
    pub fn advance(&mut self) {
        self.pos += 1;
    }

    /// We read numbers using the boolean method `is_ascii_digit()`, and if there's a dot, we also read what comes after it - don't forget about `self.advance()`.
    pub fn read_number(&mut self) -> f64 {
        let mut numbers = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() || ch == '.' {
                numbers.push(ch);
                self.advance();
            } else {
                break; // if NOT number
            }
        }

        let numbers = numbers.parse().unwrap();
        numbers
    }

    pub fn read_identifier(&mut self) -> String {
        // def
        // start: 0 (d)
        let start = self.pos;
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                self.advance();
                // e again f
            } else {
                break;
            }
        }

        self.char[start..self.pos].iter().collect()
    }

    pub fn peek_char(&self) -> Option<char> {
        self.char.get(self.pos + 1).copied()
    }
}