use crate::components::token_components::{LiteralType, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralType),
    Unary(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(left, op, right) => write!(f, "({op} {left} {right})"),
            Self::Grouping(expr) => write!(f, "(group {expr})"),
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Unary(op, right) => write!(f, "({op} {right})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expr;
    use crate::components::token_components::{LiteralType, Token, TokenType::*};

    #[test]
    fn should_stringify_deep() {
        let ast = Expr::Binary(
            Box::from(Expr::Grouping(Box::from(Expr::Unary(
                Token {
                    lexeme: "/".into(),
                    line: 1,
                    literal: None,
                    token: Slash,
                },
                Box::from(Expr::Literal(LiteralType::Number(1.0))),
            )))),
            Token {
                lexeme: "+".into(),
                line: 1,
                literal: None,
                token: Plus,
            },
            Box::from(Expr::Literal(LiteralType::Number(3.0))),
        );

        assert_eq!("(+ (group (/ 1)) 3)", ast.to_string());
    }
}
