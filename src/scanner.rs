use crate::token::Token;
use crate::tokentype::TokenType::{Eof, Identifier};
use crate::tokentype::{Literal, TokenType};
use phf;
use std::collections::HashMap;

pub(crate) struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new<'b>(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            // TODO see if we can get rid of start and current using an iterator over tokens
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // TODO scan tokens can probably be written as a single iterator
    pub(crate) fn scan_tokens(&mut self) {
        // Here we can while loop until self.current <= self.source.len(), since indices range from
        // 0 to len()-1 this looks weird, however at self.current == self.source.len(), the or part
        // of the unwrap_or in advance() is relevant, since we have reached the end of the tokens in the
        // the string. This might need tweaking later but for now it works decently with strings that represent files
        while self.current <= self.source.len() {
            let token = self.scan_token();
            self.tokens.push(token);
        }

        // crafting interpreters inserts an EOF at the end of the while loop.
        // I use unwrap_or throughout the code below so the oef is not necessary
        // self.tokens.push(Token::new(
        //     TokenType::Eof,
        //     "".parse().unwrap(),
        //     None,
        //     self.line,
        // ));
    }

    fn is_at_end(&self) -> bool {
        self.current >= (self.source.len())
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        // set the start to the start of the token
        self.start = self.current;
        let c = self.advance();
        match c {
            '(' => self.new_token(TokenType::LeftParen, None),
            ')' => self.new_token(TokenType::RightParen, None),
            '{' => self.new_token(TokenType::LeftBrace, None),
            '}' => self.new_token(TokenType::RightBrace, None),
            ',' => self.new_token(TokenType::Comma, None),
            '.' => self.new_token(TokenType::Dot, None),
            '-' => self.new_token(TokenType::Minus, None),
            '+' => self.new_token(TokenType::Plus, None),
            ';' => self.new_token(TokenType::Semicolon, None),
            '*' => self.new_token(TokenType::Star, None),
            '!' =>  {let token_type = if self.peek() == '=' {
                self.current += 1;
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
                self.new_token(token_type, None)
            },
            '=' => {
                let token_type = if self.peek() == '=' {
                    self.current += 1;
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.new_token(token_type, None)
            }
            '<' => {
                let token_type = if self.peek() == '=' {
                    self.current += 1;
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.new_token(token_type, None)
            }
            '>' => {
                let token_type = if self.peek() == '=' {
                    self.current += 1;
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.new_token(token_type, None)
            }
            '/' => self.new_token(TokenType::Slash, None),
            '"' => self.string(),
            '\0' => self.new_eof(),
            c => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                    self.error_token(format!("unexpected character '{}' ", c).parse().unwrap())
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn identifier(&mut self) -> Token {
        // iterate over the entire keyword, by doing so, we apply maximal munch
        while self.peek().is_alphabetic() {
            let c = self.advance();
        }
        // check if the word matches any of our keywords
        let text = self.source[self.start..self.current].to_string();
        let token_type = KEYWORDS
            .get(&text)
            .unwrap_or(&TokenType::Identifier)
            .clone();
        if token_type == Identifier {
            self.new_token(token_type, Some(Literal::Identifier(text)))
        } else {
            // known keywords dont need a literal value saved since we know the literal value by the keyword
            self.new_token(token_type, None)
        }
    }

    // number parses a number and saves that as a string, we delay the parsing of the number
    // to a float until later. We could already do it here, but choose not to so we can keep the
    // type of Lexeme a String instead of making it an enum of String | float64 | none
    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // consume the dot in our number
            self.advance();
        }

        while self.peek().is_digit(10) {
            self.advance();
        }

        let val = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.new_token(TokenType::Number, Some(Literal::Number(val)))
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' || self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Token::new(
                TokenType::Error,
                "unterminated string".to_string(),
                None,
                self.line,
            );
        }
        self.advance();
        // The value of the string with the starting and ending '"' trimmed
        let val = self.source[self.start + 1..self.current - 1].to_string();
        Token::new(
            TokenType::String,
            self.source[self.start..self.current].to_string(),
            Some(Literal::String(val)),
            self.line,
        )
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn new_token(&self, token_type: TokenType, literal: Option<Literal>) -> Token {
        let text = self.source[self.start..self.current].to_string();
        Token::new(token_type, text, literal, self.line)
    }

    fn new_eof(&self) -> Token {
        Token::new(TokenType::Eof, "".to_string(), None, self.line)
    }

    fn peek(&self) -> char {
        // If nth returns a None value, we are at the end of the source
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn error_token(&self, message: String) -> Token {
        Token::new(TokenType::Error, message, None, self.line)
    }
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "fun" => TokenType::Fun,
    "for" => TokenType::For,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_larger_test() {
        let source = "(()){}!*+-/=<><==={{}}\"a string is here\"randomidentifier 123.123";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        let expected = vec![
            TokenType::LeftParen,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Bang,
            TokenType::Star,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Equal,
            TokenType::Less,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::EqualEqual,
            TokenType::LeftBrace,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::RightBrace,
            TokenType::String,
            TokenType::Identifier,
            TokenType::Number,
            TokenType::Eof,
        ];

        assert_eq!(scanner.tokens.len(), expected.len());
        for (i, token) in scanner.tokens.iter().enumerate() {
            if token.kind == TokenType::String {
                assert!(token.clone()
                    .literal
                    .unwrap()
                    .eq(&Literal::String("a string is here".to_string())));
            }
            if token.kind == TokenType::Number {
                assert_eq!(token.clone().literal.unwrap(), Literal::Number(123.123))
            }
            assert_eq!(
                token.clone().kind,
                expected[i],
                "Did not find expected {:?}; {:?} was found",
                expected[i],
                token.kind
            );
        }
    }

    #[test]
    fn scanner_test() {
        let source = "//this is a comment\n(()){}//grouping stuff\n!*+-/=<><===// operators\n{{}}\"a string is here\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        let expected = vec![
            TokenType::LeftParen,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Bang,
            TokenType::Star,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Equal,
            TokenType::Less,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::EqualEqual,
            TokenType::LeftBrace,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::RightBrace,
            TokenType::String,
            TokenType::Eof,
        ];

        assert_eq!(scanner.tokens.len(), expected.len());
        for (i, token) in scanner.tokens.iter().enumerate() {
            if token.kind == TokenType::String {
                assert!(token.clone()
                    .literal
                    .unwrap()
                    .eq(&Literal::String("a string is here".to_string())));
            }
            assert_eq!(
                token.clone().kind,
                expected[i],
                "Did not find expected {:?}; {:?} was found",
                expected[i],
                token.kind
            );
        }
    }

    #[test]
    fn scanner_scans_keywords() {
        let source = "and\nclass\nelse\nfalse\nfun\nfor\nif\nnil\nor\nprint\nreturn\nsuper\nthis\ntrue\nvar\nwhile\nrandomidentifier";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        let expected = vec![
            TokenType::And,
            TokenType::Class,
            TokenType::Else,
            TokenType::False,
            TokenType::Fun,
            TokenType::For,
            TokenType::If,
            TokenType::Nil,
            TokenType::Or,
            TokenType::Print,
            TokenType::Return,
            TokenType::Super,
            TokenType::This,
            TokenType::True,
            TokenType::Var,
            TokenType::While,
            TokenType::Identifier,
            TokenType::Eof,
        ];

        for (i, token) in scanner.tokens.iter().enumerate() {
            assert_eq!(token.kind, expected[i]);
            if token.kind == TokenType::Identifier {
                assert_eq!(token.lexeme, "randomidentifier".to_string())
            }
        }
    }

    #[test]
    fn scanner_scans_strings() {
        let source = "\"blablathisisastring\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        let expected = vec![TokenType::String, TokenType::Eof];

        for (i, token) in scanner.tokens.iter().enumerate() {
            if token.kind == TokenType::String {
                assert!(token.clone()
                    .literal
                    .unwrap()
                    .eq(&Literal::String("blablathisisastring".to_string())));
            }
            assert_eq!(
                token.clone().kind,
                expected[i],
                "Did not find expected {:?}; {:?} was found with lexeme {}",
                expected[i],
                token.kind,
                token.lexeme
            );
        }
    }

    #[test]
    fn scanner_scans_numbers() {
        let source = "123.123";
        let mut scanner = Scanner::new(source);

        let expected = vec![TokenType::Number, TokenType::Eof];
        for (i, token) in scanner.tokens.iter().enumerate() {
            if token.kind == TokenType::Number {
                assert!(token.clone().literal.unwrap().eq(&Literal::Number(123.123)))
            }
            assert_eq!(
                token.clone().kind,
                expected[i],
                "Did not find expected {:?}; {:?} was found with lexeme {}",
                expected[i],
                token.kind,
                token.lexeme
            );
        }
    }

    #[test]
    fn peek_works() {
        let source = "/a|bcvd";
        let mut scanner = Scanner::new(source);
        for (i, _) in source.chars().enumerate() {
            assert_eq!(source.chars().nth(i).unwrap_or('\0'), scanner.peek());
            scanner.advance();
        }
    }

    #[test]
    fn peek_next_works() {
        let source = "/a|bcvd";
        let mut scanner = Scanner::new(source);
        for (i, _) in source.chars().enumerate() {
            assert_eq!(
                source.chars().nth(i + 1).unwrap_or('\0'),
                scanner.peek_next()
            );
            scanner.advance();
        }
    }

}
