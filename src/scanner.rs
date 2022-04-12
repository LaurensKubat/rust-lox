use crate::token::Token;
use crate::tokentype::TokenType;

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
        let c = self.advance();
        // TODO refactor the Token::new part to look nicer
        match c {
            '(' => Token::new(
                TokenType::LeftParen,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            ')' => Token::new(
                TokenType::RightParen,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '{' => Token::new(
                TokenType::LeftBrace,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '}' => Token::new(
                TokenType::RightBrace,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            ',' => Token::new(
                TokenType::Comma,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '.' => Token::new(
                TokenType::Dot,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '-' => Token::new(
                TokenType::Minus,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '+' => Token::new(
                TokenType::Plus,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            ';' => Token::new(
                TokenType::Semicolon,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '*' => Token::new(
                TokenType::Star,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '!' => Token::new(
                if self.peek() == '=' {
                    self.current += 1;
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                },
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '=' => Token::new(
                if self.peek() == '=' {
                    self.current += 1;
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                },
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '<' => Token::new(
                if self.peek() == '=' {
                    self.current += 1;
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                },
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '>' => Token::new(
                if self.peek() == '=' {
                    self.current += 1;
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                },
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '/' => Token::new(
                TokenType::Slash,
                self.source[self.start..self.current].to_string(),
                None,
                self.line,
            ),
            '\0' => Token::new(TokenType::Eof, "".to_string(), None, self.line),
            _ => self.error_token(format!("unexpected character '{}'", c).parse().unwrap()),
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

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn peek(&self) -> char {
        // If nth returns a None value, we are at the end of the source
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        c
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn error_token(&self, message: String) -> Token {
        Token::new(TokenType::Error, message, None, self.line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_test() {
        let source = "//this is a comment\n(()){}//grouping stuff\n!*+-/=<><===// operators\n{{}}";
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
            TokenType::Eof,
        ];

        assert_eq!(scanner.tokens.len(), expected.len());
        for (i, token) in scanner.tokens.iter().enumerate() {
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
