use crate::lexer::token::TokenType;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::function::Function;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::suffix::Suffix;

pub mod identifier;
pub mod integer;
pub mod suffix;
pub mod boolean;
pub mod function;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),
    Suffix(Box<Suffix>),
    Boolean(Boolean),
    Function(Function)
}

#[derive(PartialOrd, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

pub fn get_precedence(tok: TokenType) -> Precedence {
    match tok {
        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,
        TokenType::Multiply => Precedence::Product,
        TokenType::Divide => Precedence::Product,
        TokenType::LeftParenthesis => Precedence::Call,
        TokenType::Equals => Precedence::Equals,
        TokenType::LessThan => Precedence::LessGreater,
        TokenType::GreaterThan => Precedence::LessGreater,
        TokenType::LessThanOrEquals => Precedence::LessGreater,
        TokenType::GreaterThanOrEquals => Precedence::LessGreater,
        _ => Precedence::Lowest,
    }
}
