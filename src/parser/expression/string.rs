use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct LoopString {
    pub value: std::string::String,
}

pub fn parse_string_literal(p: &mut Parser) -> Option<Node> {
    let value = p.lexer.current_token.clone().unwrap().literal;

    let exp = Expression::String(LoopString { value });

    Some(Node::Expression(exp))
}
