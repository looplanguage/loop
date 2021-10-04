use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::suffix::Suffix;

pub mod identifier;
pub mod integer;
pub mod suffix;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),
    Suffix(Box<Suffix>),
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
    INDEX,
}

pub fn get_precedence(tok: TokenType) -> Precedence {
    match tok {
        TokenType::Plus => Precedence::SUM,
        TokenType::Minus => Precedence::SUM,
        TokenType::Multiply => Precedence::PRODUCT,
        TokenType::Divide => Precedence::PRODUCT,
        TokenType::LeftParenthesis => Precedence::CALL,
        TokenType::Equals => Precedence::EQUALS,
        TokenType::LessThan => Precedence::LESSGREATER,
        TokenType::GreaterThan => Precedence::LESSGREATER,
        TokenType::LessThanOrEquals => Precedence::LESSGREATER,
        TokenType::GreaterThanOrEquals => Precedence::LESSGREATER,
        _ => Precedence::LOWEST,
    }
}
