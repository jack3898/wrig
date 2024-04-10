use crate::components::token_data::TokenData;

use super::token_type::TokenType;

pub struct Scanner<'a> {
    source: Vec<char>,
    tokens: Vec<TokenType<'a>>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<TokenType> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(TokenType::EOF(TokenData {
            lexeme: "\0",
            line: self.line,
        }));

        &self.tokens
    }

    fn scan_token(&self) {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        #[cfg(debug_assertions)]
        {
            // Ideally `current` is never greater than the source length.
            // The scan should stop when it is finished.
            assert!(self.current <= self.source.len() as u32);
        }

        // There is no way the source length exceeds a 32-bit int limit!
        // It can happen in theory, but I am not worried about it.
        self.current >= self.source.len() as u32
    }
}

#[cfg(test)]
mod tests {
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
}
