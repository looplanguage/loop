use crate::lexer::token::TokenType;
use crate::parser::expression::array::Array;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::conditional::Conditional;
use crate::parser::expression::enum_expression::{enum_expression, HashableExpressionEnum};
use crate::parser::expression::float::Float;
use crate::parser::expression::function::{Call, Function};
use crate::parser::expression::hashmap::{HashableExpressionHashmap, Hashmap};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use crate::parser::expression::null::Null;
use crate::parser::expression::string::LoopString;
use crate::parser::expression::suffix::Suffix;

pub mod array;
pub mod assign_index;
pub mod boolean;
pub mod conditional;
pub mod float;
pub mod function;
pub mod hashmap;
pub mod identifier;
pub mod index;
pub mod integer;
pub mod loops;
pub mod null;
pub mod number;
pub mod string;
pub mod suffix;
pub mod enum_expression;

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
    AssignIndex(Box<AssignIndex>),
    Loop(Loop),
    LoopIterator(LoopIterator),
    LoopArrayIterator(LoopArrayIterator),
    Hashmap(Hashmap),
    Enum(enum_expression),
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
    Sqr,
    Prefix,
    Call,
    Index,
    Assign,
}

pub fn get_precedence(tok: TokenType) -> Precedence {
    match tok {
        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,
        TokenType::Multiply => Precedence::Product,
        TokenType::Divide => Precedence::Product,
        TokenType::LeftParenthesis => Precedence::Call,
        TokenType::Power => Precedence::Sqr,
        TokenType::Equals => Precedence::Equals,
        TokenType::NotEquals => Precedence::Equals,
        TokenType::LessThan => Precedence::LessGreater,
        TokenType::GreaterThan => Precedence::LessGreater,
        TokenType::LessThanOrEquals => Precedence::LessGreater,
        TokenType::GreaterThanOrEquals => Precedence::LessGreater,
        TokenType::Modulo => Precedence::Modulo,
        TokenType::Dot => Precedence::Index,
        TokenType::LeftBracket => Precedence::Index,
        TokenType::Assign => Precedence::Assign,
        _ => Precedence::Lowest,
    }
}

impl Expression {
    fn get_hash_hashmap(&self) -> Option<HashableExpressionHashmap> {
        match self {
            Expression::Integer(integer) => {
                Some(HashableExpressionHashmap::Integer(integer.clone()))
            }
            Expression::String(string) => Some(HashableExpressionHashmap::String(string.clone())),
            Expression::Boolean(boolean) => {
                Some(HashableExpressionHashmap::Boolean(boolean.clone()))
            }
            _ => None,
        }
    }

    fn get_hash_enum(&self) -> Option<HashableExpressionEnum> {
        match self {
            Expression::Identifier(identifier) => {
                Some(HashableExpressionEnum::Identifier(identifier.clone()))
            }
            _ => None,
        }
    }
}
