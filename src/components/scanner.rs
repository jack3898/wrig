use thiserror::Error;

use super::token_components::{LiteralType, Token, TokenType, TokenType::*};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<ScannerError>,
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Unexpected EOF encountered")]
    UnexpectedEof,
    #[error("Unterminated string. All strings must close. Encountered on line {line}")]
    UnterminatedString { line: usize },
    #[error("Could not convert '{received}' into a number on line {line}")]
    InvalidNumber { received: String, line: usize },
    #[error("Unexpected token '{lexeme}' on line {line}")]
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
            errors: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> (&Vec<Token>, &Vec<ScannerError>) {
        while !self.is_at_end() {
            self.start = self.current;

            let token_scan_result = self.scan_token();

            if let Err(scan_error) = token_scan_result {
                self.errors.push(scan_error);
            };
        }

        self.tokens.push(Token {
            token: EOF,
            line: self.line,
            lexeme: "\0".into(),
            literal: None,
        });

        (&self.tokens, &self.errors)
    }

    /// The source in the scanner is a collection of chars. This will grab a slice, and create a new string.
    fn get_source_slice(&self, start: usize, end: usize) -> String {
        debug_assert!(
            start != end,
            "Start index is identical to the end index when fetching the source slice"
        );

        self.source
            .get(start..end)
            .expect("Critical error in scanning source code. Attempted to extract a slice of source with an out of bounds index")
            .iter()
            .collect()
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        };

        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
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
                while !self.is_at_end() && !self.current_char_test(|c| c == '\n') {
                    self.advance();
                }
            }
            '/' => self.add_token(Slash, None),
            '"' => self.add_string_token()?,
            c if c.is_digit(10) => self.add_number_token()?,
            c if c.is_ascii_alphabetic() || c == '_' => self.add_identifier_token()?,
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            c => Err(ScannerError::UnexpectedToken {
                lexeme: c.to_string(),
                line: self.line,
            })?,
        };

        Ok(())
    }

    fn add_identifier_token(&mut self) -> Result<(), ScannerError> {
        while self.current_char_test(|c| c.is_ascii_alphanumeric()) {
            self.advance();
        }

        let text = self.get_source_slice(self.start, self.current);

        let token_type = match text.as_str() {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "true" => True,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "var" => Var,
            "while" => While,
            _ => Identifier,
        };

        self.add_token(token_type, None);

        Ok(())
    }

    /// Scans the entirety of a string literal into a token and adds it to the scanner's tokens vector.
    fn add_string_token(&mut self) -> Result<(), ScannerError> {
        while !self.current_char_test(|c| c == '"') && !self.is_at_end() {
            if self.current_char_test(|c| c == '\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString { line: self.line });
        }

        self.advance();

        let value = self.get_source_slice(self.start + 1, self.current - 1);

        self.add_token(Str, Some(LiteralType::Str(value)));

        Ok(())
    }

    fn add_number_token(&mut self) -> Result<(), ScannerError> {
        while self.current_char_test(|c| c.is_digit(10)) {
            self.advance();
        }

        if self.current_char_test(|c| c == '.') && self.next_char_test(|c| c.is_digit(10)) {
            self.advance();

            while self.current_char_test(|c| c.is_digit(10)) {
                self.advance();
            }
        }

        let source_slice = self.get_source_slice(self.start, self.current);
        let source_slice_f = source_slice
            .parse()
            .map_err(|_| ScannerError::InvalidNumber {
                received: source_slice,
                line: self.line,
            })?;

        self.add_token(Number, Some(LiteralType::Number(source_slice_f)));

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

    fn current_char_test<F>(&self, test: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        let current = self.peek();

        match current {
            Some(char) => test(char),
            None => false,
        }
    }

    fn next_char_test<F>(&self, test: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        let current = self.peek_next();

        match current {
            Some(char) => test(char),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LiteralType, Scanner, ScannerError, Token, TokenType::*};

    #[test]
    fn should_init_scanner() {
        let scanner = Scanner::new("Hello, world!");

        assert_eq!(
            scanner.source,
            vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '!']
        );
    }

    #[test]
    fn is_at_end_false_on_init() {
        let scanner = Scanner::new("Hello, world!");

        assert!(!scanner.is_at_end());
    }

    #[test]
    fn is_at_end_true_when_at_end() {
        let mut scanner = Scanner::new("Hello, world!");

        scanner.current = 13;

        assert!(scanner.is_at_end());
    }

    #[test]
    fn is_at_end_false_when_not_at_end() {
        let mut scanner = Scanner::new("Hello, world!");

        scanner.current = 12;

        assert!(!scanner.is_at_end());
    }

    #[test]
    fn should_return_error_on_unknown_token() {
        let mut scanner = Scanner::new("#");

        let result = scanner
            .scan_token()
            .expect_err("Unexpected successful scan");

        assert!(matches!(
            result,
            ScannerError::UnexpectedToken { lexeme: _, line: _ }
        ));
    }

    #[test]
    fn should_return_unexpected_eof() {
        let mut scanner = Scanner::new("");

        scanner.current = 1;

        let result = scanner.scan_token();

        assert!(matches!(result, Err(ScannerError::UnexpectedEof)));
    }

    #[test]
    fn should_add_a_token() {
        let mut scanner = Scanner::new("=");

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
        let mut scanner = Scanner::new("<=");

        let (tokens, _) = scanner.scan_tokens();

        assert_eq!(
            tokens[0],
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
        let mut scanner = Scanner::new("");

        let (tokens, _) = scanner.scan_tokens();

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
        let mut scanner = Scanner::new("// this is a comment");

        let (tokens, _) = scanner.scan_tokens();

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
        let mut scanner = Scanner::new("<= // Wow! A less-than-or-equal-to binary operator!");

        let (tokens, _) = scanner.scan_tokens();

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
        let mut scanner = Scanner::new("(\n()\n+//\r\n");

        scanner.scan_tokens();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn should_ignore_whitespace() {
        let mut scanner = Scanner::new("\t   \t  \t");

        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 1); // Includes EOF
    }

    #[test]
    fn should_scan_string() {
        let mut scanner = Scanner::new("\"Hello, world!\"");

        let (tokens, _) = scanner.scan_tokens();

        assert_eq!(
            tokens[0],
            Token {
                token: Str,
                line: 1,
                literal: Some(LiteralType::Str("Hello, world!".into())),
                lexeme: "\"Hello, world!\"".into()
            }
        );
    }

    #[test]
    fn should_error_on_unterminated_string() {
        let mut scanner = Scanner::new("\"Hello, world!");

        let (_, errors) = scanner.scan_tokens();

        assert!(matches!(
            errors[0],
            ScannerError::UnterminatedString { line: 1 }
        ));
    }

    #[test]
    fn should_convert_a_string_to_number() {
        let mut scanner = Scanner::new("3.14");

        let (tokens, _) = scanner.scan_tokens();

        assert_eq!(
            tokens[0],
            Token {
                token: Number,
                line: 1,
                literal: Some(LiteralType::Number(3.14)),
                lexeme: "3.14".into(),
            }
        );
    }

    #[test]
    fn should_convert_string_to_number_int() {
        let mut scanner = Scanner::new("3");

        let (tokens, _) = scanner.scan_tokens();

        assert_eq!(
            tokens[0],
            Token {
                token: Number,
                line: 1,
                literal: Some(LiteralType::Number(3.0)),
                lexeme: "3".into(),
            }
        );
    }

    #[test]
    fn should_add_identifier() {
        let mut scanner = Scanner::new("while");

        let (tokens, _) = scanner.scan_tokens();

        assert_eq!(
            tokens[0],
            Token {
                token: While,
                line: 1,
                literal: None,
                lexeme: "while".into(),
            }
        );
    }

    #[test]
    fn should_add_identifier_with_underscore() {
        let mut scanner = Scanner::new("_random");

        let (tokens, _) = scanner.scan_tokens();

        assert_eq!(
            tokens[0],
            Token {
                token: Identifier,
                line: 1,
                literal: None,
                lexeme: "_random".into(),
            }
        );
    }

    #[test]
    fn should_gracefully_continue_scanning_even_of_error() {
        let mut scanner = Scanner::new("() {} # $ \"hi");

        let (_, errors) = scanner.scan_tokens();

        assert!(errors.len() == 3);
    }
}
