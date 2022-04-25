use crate::token::Token;
use crate::tokentype;
use crate::tokentype::{Literal, TokenType};

pub(crate) enum Expression {
    Assign{ name: Token, expr: Box<Expression>},
    Binary{left: Box<Expression>, operator: Token, right: Box<Expression>},
    Call{callee: Box<Expression>, paren: Token, arguments: Vec<Expression>},
    Get{expr: Box<Expression>, name: Token},
    Grouping{expr: Box<Expression>},
    Literal{value: Token},
    Logical{left: Box<Expression>, operator: Token, right: Box<Expression>},
    Set{object: Box<Expression>, name: Token, value: Box<Expression>},
    Super{keyword: Token, method: Token},
    This{keyword: Token},
    Unary{operator: Token, right: Box<Expression>},
    Variable{name: Token},
}