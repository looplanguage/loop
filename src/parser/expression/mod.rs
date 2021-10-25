use crate::lexer::token::TokenType;
use crate::parser::expression::array::Array;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::conditional::Conditional;
use crate::parser::expression::float::Float;
use crate::parser::expression::function::{Call, Function};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::null::Null;
use crate::parser::expression::string::LoopString;
use crate::parser::expression::suffix::Suffix;

pub mod array;
pub mod boolean;
pub mod conditional;
pub mod float;
pub mod function;
pub mod identifier;
pub mod index;
pub mod integer;
pub mod null;
pub mod number;
pub mod string;
pub mod suffix;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),
    Suffix(Box<Suffix>),
    Boolean(Boolean),
    Function(Function),
    Conditional(Box<Conditional>),
    Null(Null),
    Call(Call),
    Float(Float),
    String(LoopString),
    Index(Box<Index>),
    Array(Box<Array>),
}

#[derive(PartialOrd, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Modulo,
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
        TokenType::NotEquals => Precedence::Equals,
        TokenType::LessThan => Precedence::LessGreater,
        TokenType::GreaterThan => Precedence::LessGreater,
        TokenType::LessThanOrEquals => Precedence::LessGreater,
        TokenType::GreaterThanOrEquals => Precedence::LessGreater,
        TokenType::Modulo => Precedence::Modulo,
        TokenType::Dot => Precedence::Index,
        TokenType::LeftBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
