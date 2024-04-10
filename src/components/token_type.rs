use std::fmt;
use std::fmt::{Debug, Formatter};

use super::token_data::TokenData;

#[derive(Debug)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParen(TokenData<'a>),
    RightParen(TokenData<'a>),
    LeftBrace(TokenData<'a>),
    RightBrace(TokenData<'a>),
    Comma(TokenData<'a>),
    Dot(TokenData<'a>),
    Minus(TokenData<'a>),
    Plus(TokenData<'a>),
    Semicolon(TokenData<'a>),
    Slash(TokenData<'a>),
    Star(TokenData<'a>),

    // One or two character tokens.
    Bang(TokenData<'a>),
    BangEqual(TokenData<'a>),
    Equal(TokenData<'a>),
    EqualEqual(TokenData<'a>),
    Greater(TokenData<'a>),
    GreaterEqual(TokenData<'a>),
    Less(TokenData<'a>),
    LessEqual(TokenData<'a>),

    // Literals.
    Identifier(TokenData<'a>),
    String(String, TokenData<'a>),
    Number(f32, TokenData<'a>),

    // Keywords.
    And(TokenData<'a>),
    Class(TokenData<'a>),
    Else(TokenData<'a>),
    False(TokenData<'a>),
    Fun(TokenData<'a>),
    For(TokenData<'a>),
    If(TokenData<'a>),
    Nil(TokenData<'a>),
    Or(TokenData<'a>),
    Print(TokenData<'a>),
    Return(TokenData<'a>),
    Super(TokenData<'a>),
    This(TokenData<'a>),
    True(TokenData<'a>),
    Var(TokenData<'a>),
    While(TokenData<'a>),

    EOF(TokenData<'a>),
}

impl fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftParen(data) => write!(f, "{}", data.to_string()),
            Self::RightParen(data) => write!(f, "{}", data.to_string()),
            Self::LeftBrace(data) => write!(f, "{}", data.to_string()),
            Self::RightBrace(data) => write!(f, "{}", data.to_string()),
            Self::Comma(data) => write!(f, "{}", data.to_string()),
            Self::Dot(data) => write!(f, "{}", data.to_string()),
            Self::Minus(data) => write!(f, "{}", data.to_string()),
            Self::Plus(data) => write!(f, "{}", data.to_string()),
            Self::Semicolon(data) => write!(f, "{}", data.to_string()),
            Self::Slash(data) => write!(f, "{}", data.to_string()),
            Self::Star(data) => write!(f, "{}", data.to_string()),
            Self::Bang(data) => write!(f, "{}", data.to_string()),
            Self::BangEqual(data) => write!(f, "{}", data.to_string()),
            Self::Equal(data) => write!(f, "{}", data.to_string()),
            Self::EqualEqual(data) => write!(f, "{}", data.to_string()),
            Self::Greater(data) => write!(f, "{}", data.to_string()),
            Self::GreaterEqual(data) => write!(f, "{}", data.to_string()),
            Self::Less(data) => write!(f, "{}", data.to_string()),
            Self::LessEqual(data) => write!(f, "{}", data.to_string()),
            Self::Identifier(data) => write!(f, "{}", data.to_string()),
            Self::String(data, string) => write!(f, "{}", data.to_string()),
            Self::Number(data, number) => write!(f, "{}", data.to_string()),
            Self::And(data) => write!(f, "{}", data.to_string()),
            Self::Class(data) => write!(f, "{}", data.to_string()),
            Self::Else(data) => write!(f, "{}", data.to_string()),
            Self::False(data) => write!(f, "{}", data.to_string()),
            Self::Fun(data) => write!(f, "{}", data.to_string()),
            Self::For(data) => write!(f, "{}", data.to_string()),
            Self::If(data) => write!(f, "{}", data.to_string()),
            Self::Nil(data) => write!(f, "{}", data.to_string()),
            Self::Or(data) => write!(f, "{}", data.to_string()),
            Self::Print(data) => write!(f, "{}", data.to_string()),
            Self::Return(data) => write!(f, "{}", data.to_string()),
            Self::Super(data) => write!(f, "{}", data.to_string()),
            Self::This(data) => write!(f, "{}", data.to_string()),
            Self::True(data) => write!(f, "{}", data.to_string()),
            Self::Var(data) => write!(f, "{}", data.to_string()),
            Self::While(data) => write!(f, "{}", data.to_string()),
            Self::EOF(data) => write!(f, "{}", data.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenData, TokenType};

    #[test]
    fn should_init_basic_token() {
        let token = TokenType::LeftParen(TokenData {
            lexeme: "(",
            line: 1,
        });

        assert_eq!(token.to_string(), "'(' on line 1")
    }

    #[test]
    fn should_init_string_token() {
        let token = TokenType::String(
            String::from("Hello, world!"),
            TokenData {
                lexeme: "\"Hello, world!\"",
                line: 1,
            },
        );

        assert_eq!(token.to_string(), "'\"Hello, world!\"' on line 1")
    }

    #[test]
    fn should_init_number_token() {
        let token = TokenType::Number(
            3.14,
            TokenData {
                lexeme: "3.14",
                line: 1,
            },
        );

        assert_eq!(token.to_string(), "'3.14' on line 1")
    }
}
