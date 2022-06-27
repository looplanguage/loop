use crate::parser::exception::SyntaxException;
use crate::parser::expression::boolean::Boolean;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::string::LoopString;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
#[allow(unused)]
pub enum HashableExpression {
    Integer(Integer),
    String(LoopString),
    Boolean(Boolean),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Hashmap {
    pub(crate) values: HashMap<HashableExpression, Expression>,
}

pub fn parse_expression_hashmap(_: &mut Parser) -> Result<Node, SyntaxException> {
    Err(SyntaxException::Unknown)
}
