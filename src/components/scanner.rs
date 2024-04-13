use std::string::String;

use thiserror::Error;

use super::token_components::{LiteralType, Token, TokenType, TokenType::*};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unexpected EOF encountered.")]
    UnexpectedEof,
    #[error("Unterminated string. All strings must close. Encountered on line {line}.")]
    UnterminatedString { line: usize },
    #[error("Could not convert {expected} into a number on line {line}.")]
    InvalidNumber { expected: String, line: usize },
    #[error("Unexpected token {lexeme} on line {line}.")]
    UnexpectedToken { lexeme: String, line: usize },
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token()?;
        }

        self.tokens.push(Token {
            token: EOF,
            line: self.line,
            lexeme: "\0".into(),
            literal: None,
        });

        Ok(&self.tokens)
    }

    /// The source in the scanner is a collection of chars. This will grab a slice, and create a new string.
    fn get_source_slice(&self, start: usize, end: usize) -> String {
        debug_assert!(
            start != end,
            "Start index is identical to the end index when fetching the source slice."
        );

        self.source
            .get(start..end)
            .expect("Critical error in scanning source code. Attempted to extract a slice of source with an out of bounds index.")
            .iter()
            .collect()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        };

        self.source.get(self.current as usize).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let current_char = self.peek()?;

        self.current += 1;

        Some(current_char)
    }

    /// Advances the scanner's `current` only if the test character matches the current character.
    /// Otherwise false is returned and the `current` is not advanced.
    fn conditional_advance(&mut self, test: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        let current_char = self.peek();

        if let Some(char) = current_char {
            self.current += 1;

            char == test
        } else {
            false
        }
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        let c = self.advance().ok_or(ScannerError::UnexpectedEof)?;

        match c {
            '(' => self.add_token(LeftParen, None),
            ')' => self.add_token(RightParen, None),
            '{' => self.add_token(LeftBrace, None),
            '}' => self.add_token(RightBrace, None),
            ',' => self.add_token(Comma, None),
            '.' => self.add_token(Dot, None),
            '-' => self.add_token(Minus, None),
            '+' => self.add_token(Plus, None),
            ';' => self.add_token(Semicolon, None),
            '*' => self.add_token(Star, None),
            '=' if self.conditional_advance('=') => self.add_token(EqualEqual, None),
            '!' if self.conditional_advance('=') => self.add_token(BangEqual, None),
            '<' if self.conditional_advance('=') => self.add_token(LessEqual, None),
            '>' if self.conditional_advance('=') => self.add_token(GreaterEqual, None),
            '=' => self.add_token(Equal, None),
            '/' if self.conditional_advance('/') => {
                // This is a comment, like this one! We'll just strip it.
                while !self.is_at_end() && !self.is_at_end_of_line() {
                    self.advance();
                }
            }
            '/' => self.add_token(Slash, None),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            c => Err(ScannerError::UnexpectedToken {
                lexeme: c.to_string(),
                line: self.line,
            })?,
        };

        Ok(())
    }

    /// Adds the token to the tokens vector.
    /// Always advances the `current`.
    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralType>) {
        let lexeme = self.get_source_slice(self.start, self.current);

        self.tokens.push(Token {
            line: self.line,
            lexeme: lexeme.into(),
            token: token_type,
            literal,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_at_end_of_line(&self) -> bool {
        let current = self.peek();

        match current {
            Some(char) => char == '\n',
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ScannerError, Token, TokenType::*};

    #[test]
    fn should_init_scanner() {
        let scanner = super::Scanner::new("Hello, world!");

        assert_eq!(
            scanner.source,
            vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '!']
        );
    }

    #[test]
    fn is_at_end_false_on_init() {
        let scanner = super::Scanner::new("Hello, world!");

        assert!(!scanner.is_at_end());
    }

    #[test]
    fn is_at_end_true_when_at_end() {
        let mut scanner = super::Scanner::new("Hello, world!");

        scanner.current = 13;

        assert!(scanner.is_at_end());
    }

    #[test]
    fn is_at_end_false_when_not_at_end() {
        let mut scanner = super::Scanner::new("Hello, world!");

        scanner.current = 12;

        assert!(!scanner.is_at_end());
    }

    #[test]
    fn should_return_error_on_unknown_token() {
        let mut scanner = super::Scanner::new("Hello, world!");

        let result = scanner.scan_token();

        assert!(matches!(
            result,
            Err(ScannerError::UnexpectedToken { lexeme: _, line: _ })
        ));
    }

    #[test]
    fn should_return_unexpected_eof() {
        let mut scanner = super::Scanner::new("");

        scanner.current = 1;

        let result = scanner.scan_token();

        assert!(matches!(result, Err(ScannerError::UnexpectedEof)));
    }

    #[test]
    fn should_add_a_token() {
        let mut scanner = super::Scanner::new("=");

        // This would be handled automatically in the scan_tokens method, but for testing purposes we need to set the start and current manually.
        scanner.current += 1;

        scanner.add_token(LeftParen, None);

        assert_eq!(
            scanner.tokens,
            vec![Token {
                token: LeftParen,
                line: 1,
                literal: None,
                lexeme: "=".into()
            }]
        )
    }

    #[test]
    fn should_add_two_char_token() {
        let mut scanner = super::Scanner::new("<=");

        let token = scanner.scan_tokens().unwrap();

        assert_eq!(
            token[0],
            Token {
                token: LessEqual,
                line: 1,
                literal: None,
                lexeme: "<=".into()
            }
        );
    }

    #[test]
    fn should_add_eof_after_scan() {
        let mut scanner = super::Scanner::new("");

        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(
            tokens[0],
            Token {
                token: EOF,
                line: 1,
                literal: None,
                lexeme: "\0".into()
            }
        );
    }

    #[test]
    fn should_strip_comments() {
        let mut scanner = super::Scanner::new("// this is a comment");

        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(
            tokens[0],
            Token {
                token: EOF,
                line: 1,
                literal: None,
                lexeme: "\0".into()
            }
        );
        assert!(tokens.len() == 1);
    }

    #[test]
    fn should_match_tokens_prior_to_comment() {
        let mut scanner =
            super::Scanner::new("<= // Wow! A less-than-or-equal-to binary operator!");

        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(
            tokens[0],
            Token {
                token: LessEqual,
                line: 1,
                literal: None,
                lexeme: "<=".into()
            }
        );
        assert!(tokens.len() == 2); // Includes EOF
    }

    #[test]
    fn should_increment_line() {
        let mut scanner = super::Scanner::new("(\n()\n+//\n");

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn should_ignore_whitespace() {
        let mut scanner = super::Scanner::new("\t   \t  \t");

        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 1); // Includes EOF
    }
}
