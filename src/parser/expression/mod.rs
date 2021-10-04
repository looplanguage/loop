use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;

pub mod identifier;
pub mod integer;

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),
}

#[derive(PartialOrd, PartialEq)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
    INDEX
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
        _ => Precedence::LOWEST
    }
}
