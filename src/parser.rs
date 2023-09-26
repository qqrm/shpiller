//! Parser module for handling tokens and constructing nodes.

use crate::Token;

/// Represents an expression node that contains an integer value.
///
/// # Example
///
/// ```
/// let expr = NodeExpr { int_value: 5 };
/// ```
pub struct NodeExpr {
    pub int_value: i64,
}

/// Represents an exit node that contains an expression.
///
/// # Example
///
/// ```
/// let exit = NodeExit { expr: NodeExpr { int_value: 5 } };
/// ```
pub struct NodeExit {
    pub expr: NodeExpr,
}

/// `Parser` is responsible for parsing a list of tokens and constructing the appropriate node structures.
pub(crate) struct Parser {
    tokens: Vec<Token>, // List of tokens to be parsed
    index: usize,       // Index pointing to the current token
}

impl Parser {
    /// Constructs a new `Parser` instance.
    ///
    /// # Parameters
    ///
    /// * `tokens`: A vector of tokens to be parsed.
    pub fn new(tokens: Vec<Token>) -> Parser {
        Self { tokens, index: 0 }
    }

    /// Returns a token `ahead` positions from the current index.
    ///
    /// # Parameters
    ///
    /// * `ahead`: Number of positions ahead of the current token.
    fn peek(&self, ahead: usize) -> Option<Token> {
        if self.index + ahead >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.index + ahead])
        }
    }

    /// Utility function to peek at the current token.
    fn peek_one(&self) -> Option<Token> {
        self.peek(0)
    }

    /// Consumes the current token and advances to the next one.
    ///
    /// # Returns
    ///
    /// A reference to the token that was just consumed.
    fn consume(&mut self) -> &Token {
        self.index += 1;
        &self.tokens[self.index - 1]
    }

    /// Parses the tokens into a `NodeExit` structure.
    ///
    /// # Returns
    ///
    /// An `Option` containing a `NodeExit` structure if parsing is successful, or `None` if not.
    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node = None;

        // Iterating and processing tokens based on their type.
        while let Some(token) = self.peek_one() {
            match token {
                Token::Exit => {
                    self.consume();
                    if let Some(node) = self.parse_expression() {
                        exit_node = Some(NodeExit { expr: node });
                        if let Some(Token::Semicolon) = self.peek_one() {
                            self.consume();
                        } else {
                            panic!("expected semicolon after expression");
                        }
                    } else {
                        panic!("invalid expression after exit token");
                    }
                }
                _ => {
                    self.consume(); // consume other tokens to avoid infinite loop
                }
            }
        }

        self.index = 0;
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
        if let Some(Token::IntLiteral(int_value)) = self.peek_one() {
            self.consume();
            Some(NodeExpr { int_value })
        } else {
            None
        }
    }
}
