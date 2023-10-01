/// Represents the types of tokens that can be produced by the lexer.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Represents the 'return' keyword.
    Exit,
    /// Represents an integer literal.
    IntLiteral(i64),
    /// Represents the ';' symbol.
    Semicolon,
    /// Represents the '(' symbol.
    OpenParen,
    /// Represents the ')' symbol.
    CloseParen,
    /// Represents the 'let' keyword
    Let,
    /// Represents the '=' symbol
    Equals,
    /// Represents variable names like 'x'
    Identifier(String),
}

pub struct Tokenizer {
    src: Vec<char>,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Tokenizer {
            src: source.chars().collect(),
        }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for ch in &self.src {
            match ch {
                ' ' | ';' | '(' | ')' | '=' | '\n' => {
                    if !current_token.is_empty() {
                        if current_token == "exit" {
                            tokens.push(Token::Exit);
                        } else if current_token == "let" {
                            tokens.push(Token::Let);
                        } else if let Ok(value) = current_token.parse::<i64>() {
                            tokens.push(Token::IntLiteral(value));
                        } else {
                            tokens.push(Token::Identifier(current_token.trim().to_string()));
                        }
                        current_token.clear();
                    }

                    match ch {
                        ';' => tokens.push(Token::Semicolon),
                        '(' => tokens.push(Token::OpenParen),
                        ')' => tokens.push(Token::CloseParen),
                        '=' => tokens.push(Token::Equals),
                        _ => {}
                    }
                }
                _ => current_token.push(*ch),
            }
        }

        dbg!(&tokens);

        if !current_token.is_empty() {
            if current_token == "exit" {
                tokens.push(Token::Exit);
            } else if current_token == "let" {
                tokens.push(Token::Let);
            } else if let Ok(value) = current_token.parse::<i64>() {
                tokens.push(Token::IntLiteral(value));
            } else {
                tokens.push(Token::Identifier(current_token.trim().to_string()));
            }
        }

        tokens
    }
}
