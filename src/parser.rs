//! Parser module for handling tokens and constructing nodes.
//!
//! The parser module provides structures and methods to transform a sequence of tokens into
//! more abstract representations like expressions or other nodes.

use crate::Token;
use std::collections::HashMap;

/// Represents an expression node that contains an integer value.
///
/// The `NodeExpr` struct is used to represent literal integer values in the abstract syntax tree.
///
/// # Examples
///
/// ```
/// let expr = NodeExpr { int_value: 5 };
/// ```
pub struct NodeExpr {
    pub int_value: i64,
}

/// Represents an exit node that contains an expression.
///
/// The `NodeExit` struct is used to represent exit nodes in the abstract syntax tree.
///
/// # Examples
///
/// ```
/// let exit = NodeExit { expr: NodeExpr { int_value: 5 } };
/// ```
pub struct NodeExit {
    pub expr: NodeExpr,
}

/// `Parser` is responsible for parsing a list of tokens and constructing the appropriate node structures.
///
/// The parser uses methods like `peek` and `consume` to iterate through the list of tokens and uses
/// recursive-descent parsing techniques to create the abstract syntax tree.
///
/// # Examples
///
/// ```
/// let tokens = // ... generate or provide list of tokens
/// let mut parser = Parser::new(tokens);
/// let ast = parser.parse();
/// ```
pub(crate) struct Parser {
    tokens: Vec<Token>, // List of tokens to be parsed
    index: usize,       // Index pointing to the current token
    symbol_table: HashMap<String, i64>,
}

impl Parser {
    /// Constructs a new `Parser` instance with a given list of tokens.
    ///
    /// # Parameters
    ///
    /// * `tokens`: A vector of tokens to be parsed.
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self {
            tokens,
            index: 0,
            symbol_table: HashMap::new(),
        }
    }

    /// Returns a token `ahead` positions from the current index.
    ///
    /// This method provides a way to look ahead in the token list without consuming any tokens.
    ///
    /// # Parameters
    ///
    /// * `ahead`: Number of positions ahead of the current token.
    fn peek(&self, ahead: usize) -> Option<Token> {
        if self.index + ahead >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.index + ahead].clone())
        }
    }

    /// Utility function to peek at the current token without advancing the index.
    fn peek_current(&self) -> Option<Token> {
        self.peek(0)
    }

    /// Consumes the current token and advances to the next one.
    ///
    /// This method allows the parser to advance through the list of tokens.
    ///
    /// # Returns
    ///
    /// The token that was just consumed.
    fn consume(&mut self) -> Token {
        self.index += 1;
        self.tokens[self.index - 1].clone()
    }

    fn parse_assignment(&mut self) {
        // Since we already know the current token is `Let`, we don't need to check it again.
        // So, just consume it.
        self.consume();

        // Expect and consume identifier for the variable name
        let var_name = match self.consume() {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier for variable name."),
        };

        // Expect and consume the equals sign
        match self.consume() {
            Token::Equals => (),
            _ => panic!("Expected equals sign after identifier."),
        }

        // Expect and consume the integer literal (value assigned to the variable)
        let value = match self.consume() {
            Token::IntLiteral(val) => val,
            _ => panic!("Expected an integer after equals sign."),
        };

        // Insert the variable and its value into the symbol table
        self.symbol_table.insert(var_name.to_string(), value);
    }

    /// Parses the tokens into a `NodeExit` structure.
    ///
    /// # Returns
    ///
    /// An `Option` containing a `NodeExit` structure if parsing is successful, or `None` if not.
    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node = None;

        while let Some(token) = self.peek_current() {
            match token {
                Token::Let => {
                    self.parse_assignment();
                }
                Token::Exit => {
                    self.consume();
                    match self.consume() {
                        Token::OpenParen => (),
                        _ => panic!("Expected an open parenthesis after 'exit'."),
                    }

                    let expr = self.parse_expression();

                    match self.consume() {
                        Token::CloseParen => (),
                        _ => panic!("Expected a close parenthesis after expression."),
                    }

                    exit_node = Some(NodeExit {
                        expr: expr.unwrap(),
                    }); // You might want to handle this unwrap more gracefully.
                }
                _ => {
                    self.consume();
                }
            }
        }

        exit_node.or(Some(NodeExit {
            expr: NodeExpr { int_value: 0 },
        }))
    }

    /// Parses an expression from the tokens.
    ///
    /// # Returns
    ///
    /// An `Option` containing a `NodeExpr` if parsing is successful, or `None` if not.
    fn parse_expression(&mut self) -> Option<NodeExpr> {
        match self.peek_current() {
            Some(Token::IntLiteral(int_value)) => {
                self.consume();
                Some(NodeExpr { int_value })
            }
            Some(Token::Identifier(name)) => {
                self.consume();
                if let Some(value) = self.symbol_table.get(&name) {
                    Some(NodeExpr { int_value: *value })
                } else {
                    panic!("Undefined variable: {}", name);
                }
            }
            _ => None,
        }
    }
}
