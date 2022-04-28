use crate::ast::Expression;
use crate::token::Token;
use crate::tokentype::TokenType;

#[derive(Debug)]
pub(crate) enum Error {
    MissingToken(TokenType),
    UnexpectedToken(Token),
}

type Errors = Vec<Error>;

pub struct Interpreter {
    tokens: Vec<Token>,
    current: usize,
}

impl Interpreter {
    pub(crate) fn new(tokens: Vec<Token>) -> Interpreter {
        Interpreter { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Result<Expression, Errors> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expression, Errors> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Errors> {
        // if our 'left' side is an error, parse the right and try to find errors there
        let mut expr = self.comparison();
        //TODO move the is_err part to a separate function
        if expr.is_err() {
            let mut errors = expr.unwrap_err();
            // move to the next good spot to start parsing
            self.synchronize();
            // parse the next part
            let right = self.expression();
            // add any errors from the right part to the current errors
            if right.is_err() {
                for error in right.unwrap_err() {
                    errors.push(error)
                }
            }
            return Err(errors);
        }

        let mut res = expr.unwrap();

        while self.check_and_consume(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            // since left is known to be error free, we can parse right and return the error if we find one
            let right = self.comparison()?;
            res = Expression::Binary {
                left: Box::new(res),
                operator,
                right: Box::new(right),
            };
        }
        Ok(res)
    }

    fn comparison(&mut self) -> Result<Expression, Errors> {
        let mut expr = self.term();
        if expr.is_err() {
            let mut errors = expr.unwrap_err();
            self.synchronize();
            let right = self.expression();
            if right.is_err() {
                for error in right.unwrap_err() {
                    errors.push(error)
                }
            }
            return Err(errors);
        }

        let mut res = expr.unwrap();

        while self.check_and_consume(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            res = Expression::Binary {
                left: Box::new(res),
                operator,
                right: Box::new(right),
            }
        }
        return Ok(res);
    }

    fn term(&mut self) -> Result<Expression, Errors> {
        let mut expr = self.factor();
        if expr.is_err() {
            let mut errors = expr.unwrap_err();
            self.synchronize();
            let right = self.expression();
            if right.is_err() {
                for error in right.unwrap_err() {
                    errors.push(error)
                }
            }
            return Err(errors);
        }

        let mut res = expr.unwrap();

        while self.check_and_consume(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            res = Expression::Binary {
                left: Box::new(res),
                operator,
                right: Box::new(right),
            }
        }
        return Ok(res);
    }

    fn factor(&mut self) -> Result<Expression, Errors> {
        let expr = self.unary();
        if expr.is_err() {
            let mut errors = expr.unwrap_err();
            self.synchronize();
            let right = self.expression();
            if right.is_err() {
                for error in right.unwrap_err() {
                    errors.push(error)
                }
            }
            return Err(errors);
        }

        let mut res = expr.unwrap();

        while self.check_and_consume(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            res = Expression::Binary {
                left: Box::new(res),
                operator,
                right: Box::new(right),
            }
        }
        return Ok(res);
    }

    fn unary(&mut self) -> Result<Expression, Errors> {
        if self.check_and_consume(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expression, Errors> {
        if self.check_and_consume(&[TokenType::False]) {
            return Ok(Expression::Literal {
                value: self.previous().clone(),
            });
        }
        if self.check_and_consume(&[TokenType::True]) {
            return Ok(Expression::Literal {
                value: self.previous().clone(),
            });
        }
        if self.check_and_consume(&[TokenType::Nil]) {
            return Ok(Expression::Literal {
                value: self.previous().clone(),
            });
        }

        if self.check_and_consume(&[TokenType::Number, TokenType::String]) {
            return Ok(Expression::Literal {
                value: self.previous().clone(),
            });
        }

        if self.check_and_consume(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen)?;
            return Ok(Expression::Grouping {
                expr: Box::new(expr),
            });
        }
        Err(vec![Error::UnexpectedToken(self.peek().clone())])
    }

    // same as match from the book, however match is reserved in rust
    // checks whether any of the token_types match the current token
    fn check_and_consume(&mut self, token_types: &[TokenType]) -> bool {
        for tt in token_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().kind == token_type
    }

    fn consume(&mut self, expected: TokenType) -> Result<&Token, Errors> {
        if self.peek().kind == expected {
            return Ok(self.advance());
        }
        Err(vec![Error::MissingToken(expected)])
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }
        }

        match self.peek().kind {
            TokenType::Class => return,
            TokenType::Fun => return,
            TokenType::Var => return,
            TokenType::For => return,
            TokenType::If => return,
            TokenType::While => return,
            TokenType::Print => return,
            TokenType::Return => return,
            _ => (),
        }
        self.advance();
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // peek should never panic on unwrap, since we only iterate over the indices of the vector
    // if we were exposing advance and previous, this should have some more checks or a default value
    fn peek(&self) -> &Token {
        return self.tokens.get(self.current).unwrap();
    }

    // previous should never panic on unwrap, since we only iterate over the indices of the vector
    // if we were exposing advance and previous, this should have some more checks or a default value
    fn previous(&self) -> &Token {
        return self.tokens.get(self.current - 1).unwrap();
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }
}
