use crate::tokentype::{Literal, TokenType};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Literal>,
    pub(crate) line: usize,
    // TODO add column and length of token for better error handling
}

impl Token {
    pub(crate) fn new(
        kind: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(literal) = &self.literal {
            write!(f, "{}", literal)
        } else {
            write!(f, "{}", self.lexeme)
        }
    }
}
