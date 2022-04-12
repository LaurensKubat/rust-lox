use crate::tokentype::TokenType;

#[derive(Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<String>,
    pub(crate) line: usize,
    // TODO add column and length of token for better error handling
}

impl Token {
    pub(crate) fn new(
        kind: TokenType,
        lexeme: String,
        literal: Option<String>,
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

impl ToString for Token {
    fn to_string(&self) -> String {
        if let Some(literal) = &self.literal {
            format!("{:?} {} {}", self.kind, self.lexeme, literal)
        } else {
            format!("{:?} {}", self.kind, self.lexeme)
        }
    }
}
