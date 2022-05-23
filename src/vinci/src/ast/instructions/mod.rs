use crate::ast::instructions::conditional::Conditional;
use crate::ast::instructions::function::{Call, Function, LibCall};
use crate::ast::instructions::memory::{Copy, Index, Load, LoadLib, Push, Slice, Store};
use crate::ast::instructions::suffix::Suffix;
use crate::ast::instructions::while_loop::While;
use crate::types::ValueType;
use std::fmt::{Debug, Display, Formatter};

pub mod conditional;
pub mod function;
pub mod memory;
pub mod suffix;
pub mod while_loop;

#[derive(PartialEq, Clone)]
pub enum Node {
    CONSTANT(ValueType),
    LOAD(Load),
    STORE(Store),
    SUFFIX(Box<Suffix>),
    CONDITIONAL(Box<Conditional>),
    FUNCTION(Box<Function>),
    CALL(Box<Call>),
    // TODO: This should not exist but, because Node::CALL was annoying this is here
    LIBCALL(LibCall),
    WHILE(Box<While>),
    INDEX(Index),
    SLICE(Slice),
    PUSH(Push),
    COPY(Copy),
    LOADLIB(LoadLib),
    RETURN(Box<Node>),
    ASSIGN(Box<Node>, Box<Node>),
    POP(Box<Node>, Box<Node>),
    LENGTH(Box<Node>),
    AND(Box<Node>, Box<Node>),
    OR(Box<Node>, Box<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::LOAD(load) => write!(f, "{}", load),
            Node::SUFFIX(suffix) => write!(f, "{}", suffix),
            Node::STORE(store) => write!(f, "{}", store),
            Node::CONSTANT(value_type) => write!(f, "{}", value_type),
            Node::CONDITIONAL(conditional) => write!(f, "{}", conditional),
            Node::FUNCTION(func) => write!(f, "{}", func),
            Node::CALL(call) => write!(f, "{}", call),
            Node::WHILE(wh) => write!(f, "{}", wh),
            Node::PUSH(wh) => write!(f, "{}", wh),
            Node::SLICE(wh) => write!(f, "{}", wh),
            Node::INDEX(wh) => write!(f, "{}", wh),
            Node::LOADLIB(loadlib) => write!(f, "{}", loadlib),
            Node::COPY(copy) => write!(f, "{}", copy),
            Node::RETURN(ret) => write!(f, "{}", ret),
            Node::POP(a, b) => write!(f, "{}, {}", a, b),
            Node::ASSIGN(a, b) => write!(f, "{}, {}", a, b),
            Node::LENGTH(a) => write!(f, "{}", a),
            Node::AND(a, b) => write!(f, "{}, {}", a, b),
            Node::OR(a, b) => write!(f, "{}, {}", a, b),
            Node::LOADLIB(a) => write!(f, "{}", a),
            Node::LIBCALL(a) => write!(f, "{}", a),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
