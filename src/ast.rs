use crate::token::Token;
use crate::tokentype;
use crate::tokentype::{Literal, TokenType};
use std::fmt::{write, Display, Formatter};

pub(crate) enum Expression {
    Assign {
        name: Token,
        expr: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Get {
        expr: Box<Expression>,
        name: Token,
    },
    Grouping {
        expr: Box<Expression>,
    },
    Literal {
        value: Token,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Set {
        object: Box<Expression>,
        name: Token,
        value: Box<Expression>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        name: Token,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Assign { name, expr } => todo!(),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
            Expression::Call { .. } => todo!(),
            Expression::Get { .. } => todo!(),
            Expression::Grouping { expr } => write!(f, "(group {})", expr),
            Expression::Literal { value } => write!(f, "{}", value.literal.clone().unwrap()),
            Expression::Logical { .. } => todo!(),
            Expression::Set { .. } => todo!(),
            Expression::Super { .. } => todo!(),
            Expression::This { .. } => todo!(),
            Expression::Unary { operator, right } => write!(f, "({}, {})", operator.lexeme, right),
            Expression::Variable { .. } => todo!(),
        }
    }
}

// instinctively, we want to replace visitor with an iterator and fold the expression to get the string as we can do in Haskell
// however this does not sit well with rust, this approach works a bit easier.
impl Visitor<String> for Expression {
    fn visit(&self) -> String {
        self.to_string()
    }
}

pub(crate) trait Visitor<T> {
    fn visit(&self) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor_string() {
        let test = Expression::Binary {
            left: Box::new(Expression::Binary {
                left: Box::new(Expression::Literal {
                    value: Token::new(
                        TokenType::Number,
                        "123".to_string(),
                        Some(Literal::Number(123 as f64)),
                        0,
                    ),
                }),
                operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
                right: Box::new(Expression::Literal {
                    value: Token::new(
                        TokenType::Number,
                        "321".to_string(),
                        Some(Literal::Number(321 as f64)),
                        0,
                    ),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 0),
            right: Box::new(Expression::Grouping {
                expr: Box::new(Expression::Literal {
                    value: Token::new(
                        TokenType::Number,
                        "234".to_string(),
                        Some(Literal::Number(234 as f64)),
                        0,
                    ),
                }),
            }),
        };

        assert_eq!(test.visit(), "(* (+ 123 321) (group 234))")

    }
}
