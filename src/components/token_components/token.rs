use std::fmt::Display;

use super::LiteralType;
use super::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: TokenType,
    pub line: usize,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
