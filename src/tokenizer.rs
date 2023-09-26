const EXIT: &str = "exit";

/// Represents the types of tokens that can be produced by the lexer.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    /// Represents the 'return' keyword.
    Exit,
    /// Represents an integer literal.
    IntLiteral(i64),
    /// Represents the ';' symbol.
    Semicolon,
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

    /// Tokenize the given input into a list of `TokenType` values.
    ///
    /// # Returns
    ///
    /// A vector of tokens representing the source code.
    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for ch in &self.src {
            match ch {
                ' ' | ';' => {
                    if !current_token.is_empty() {
                        if current_token == EXIT {
                            tokens.push(Token::Exit);
                        } else if let Ok(value) = current_token.parse::<i64>() {
                            tokens.push(Token::IntLiteral(value));
                        }

                        current_token.clear();
                    }

                    if *ch == ';' {
                        tokens.push(Token::Semicolon);
                    }
                }
                _ => current_token.push(*ch),
            }
        }

        if !current_token.is_empty() {
            if current_token == EXIT {
                tokens.push(Token::Exit);
            } else if let Ok(value) = current_token.parse::<i64>() {
                tokens.push(Token::IntLiteral(value));
            }
        }

        tokens
    }
}
