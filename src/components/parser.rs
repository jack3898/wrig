use thiserror::Error;

use super::{
    parser_components::Expr,
    token_components::{
        LiteralType, Token,
        TokenType::{self, *},
    },
};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Unexpected token '{found}', expected {expected} on line {line}")]
    UnexpectedToken {
        found: Token,
        expected: Token,
        line: usize,
    },
    #[error("Parse error. {message}")]
    ParseError { message: String },
    #[error("No primary found on line {line}")]
    PrimaryError { line: usize },
    #[error("No literal type found on line {line}")]
    UndefinedLiteral { line: usize },
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

trait ASTOperations {
    fn expression(&mut self) -> Result<Expr, ParserError>;
    fn equality(&mut self) -> Result<Expr, ParserError>;
    fn comparison(&mut self) -> Result<Expr, ParserError>;
    fn term(&mut self) -> Result<Expr, ParserError>;
    fn factor(&mut self) -> Result<Expr, ParserError>;
    fn unary(&mut self) -> Result<Expr, ParserError>;
    fn primary(&mut self) -> Result<Expr, ParserError>;
}

impl<'a> ASTOperations for Parser<'a> {
    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;
        let token_types = [BangEqual, EqualEqual];

        while self.match_token_type(&token_types) {
            let op = self.previous().clone();
            let right_expr = self.comparison()?;

            expr = Expr::Binary(Box::from(expr), op, Box::from(right_expr))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        let token_types = [Greater, GreaterEqual, Less, LessEqual];

        while self.match_token_type(&token_types) {
            let op = self.previous().clone();
            let right_expr = self.term()?;

            expr = Expr::Binary(Box::from(expr), op, Box::from(right_expr));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;
        let token_types = [Minus, Plus];

        while self.match_token_type(&token_types) {
            let op = self.previous().clone();
            let right_expr = self.factor()?;

            expr = Expr::Binary(Box::from(expr), op, Box::from(right_expr));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;
        let token_types = [Slash, Star];

        while self.match_token_type(&token_types) {
            let op = self.previous().clone();
            let right_expr = self.unary()?;

            expr = Expr::Binary(Box::from(expr), op, Box::from(right_expr));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        let token_types = [Bang, Minus];

        if self.match_token_type(&token_types) {
            let op = self.previous().clone();
            let right_expr = self.unary()?;

            return Ok(Expr::Unary(op, Box::from(right_expr)));
        }

        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token_type(&[False]) {
            return Ok(Expr::Literal(LiteralType::Bool(false)));
        }

        if self.match_token_type(&[True]) {
            return Ok(Expr::Literal(LiteralType::Bool(true)));
        }

        if self.match_token_type(&[Nil]) {
            return Ok(Expr::Literal(LiteralType::Nil));
        }

        if self.match_token_type(&[Number, Str]) {
            return Ok(Expr::Literal(self.previous().literal.clone().unwrap()));
        }

        if self.match_token_type(&[LeftParen]) {
            let expr = self.expression()?;

            self.consume(RightParen, "Expected ')' after expression".into())?;

            return Ok(Expr::Grouping(Box::from(expr)));
        }

        Err(ParserError::PrimaryError {
            line: self.peek().line,
        })
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        on_fail_msg: String,
    ) -> Result<&Token, ParserError> {
        if self.match_token_type(&[token_type]) {
            return Ok(self.advance());
        }

        Err(ParserError::ParseError {
            message: on_fail_msg,
        })
    }

    #[allow(dead_code)]
    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token == TokenType::Semicolon {
                return;
            }

            match self.peek().token {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => {
                    self.advance();

                    return;
                }
            };
        }
    }

    fn match_token_type(&mut self, check_token_types: &[TokenType]) -> bool {
        for token in check_token_types {
            if self.current_eq(*token) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn current_eq(&self, check_token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token == check_token
    }

    fn is_at_end(&self) -> bool {
        self.peek().token == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        debug_assert!(
            self.current < self.tokens.len(),
            "Attempt to get token using an out of bounds index"
        );

        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        let index = self.current - 1;

        debug_assert!(
            index < self.tokens.len(),
            "Attempt to get previous token using an out of bounds index"
        );

        self.tokens.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::components::{
        parser::ParserError,
        token_components::{LiteralType, Token, TokenType::*},
        Scanner,
    };

    #[test]
    fn should_add() {
        let one = Token {
            lexeme: "1".into(),
            line: 1,
            literal: Some(LiteralType::Number(1.0)),
            token: Number,
        };

        let plus = Token {
            lexeme: "+".into(),
            line: 1,
            literal: None,
            token: Plus,
        };

        let two = Token {
            lexeme: "2".into(),
            line: 1,
            literal: Some(LiteralType::Number(2.0)),
            token: Number,
        };

        let semi = Token {
            lexeme: ';'.into(),
            line: 1,
            literal: None,
            token: Semicolon,
        };

        let scanned_tokens = vec![one, plus, two, semi];

        let mut parser = Parser::new(&scanned_tokens);
        let expr = parser.parse();

        assert_eq!(expr.unwrap().to_string(), "(+ 1 2)");
    }

    #[test]
    fn input_from_scanner() {
        let mut scanner = Scanner::new("1 + 2 <= 5 + 7");
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse();

        assert_eq!(expr.unwrap().to_string(), "(<= (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn should_report_paren_error() {
        let mut scanner = Scanner::new("1 + 2 + (5 + 7");
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(&tokens);
        let expr_err = parser.parse().expect_err("Successfully parsed");

        assert!(matches!(expr_err, ParserError::ParseError { message: _ }))
    }

    #[test]
    fn should_report_primary_error() {
        let mut scanner = Scanner::new("class + 2 + 1");
        let (tokens, _) = scanner.scan_tokens();
        let mut parser = Parser::new(&tokens);
        let expr_err = parser.parse().expect_err("Successfully parsed");

        assert!(matches!(expr_err, ParserError::PrimaryError { line: _ }))
    }
}
