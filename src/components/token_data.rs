use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct TokenData<'a> {
    pub lexeme: &'a str,
    pub line: i32,
}

impl fmt::Display for TokenData<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lexeme = &self.lexeme;
        let line = self.line;

        write!(f, "'{lexeme}' on line {line}")
    }
}
