use crate::lexer::token::TokenType;
use crate::parser::expression::array::Array;
use crate::parser::expression::assign_index::AssignIndex;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::conditional::Conditional;
use crate::parser::expression::float::Float;
use crate::parser::expression::function::{Call, Function};
use crate::parser::expression::hashmap::Hashmap;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::index::Index;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use crate::parser::expression::null::Null;
use crate::parser::expression::string::LoopString;
use crate::parser::expression::suffix::Suffix;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
    Assign,
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
        TokenType::Assign => Precedence::Assign,
        _ => Precedence::Lowest,
    }
}

impl Expression {
    fn get_hash(&self) -> Option<u64> {
        let mut s = DefaultHasher::new();

        match self {
            Expression::Integer(integer) => {
                integer.value.hash(&mut s);
            }
            Expression::Boolean(boolean) => {
                boolean.value.hash(&mut s);
            }
            Expression::String(string) => {
                string.value.hash(&mut s)
            }
            _ => {
                return None;
            }
        }

        Some(s.finish())
    }
}
