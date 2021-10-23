use crate::lexer::token::TokenType;
use crate::parser::expression::float::parse_float_literal;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn find_extension(&self, name: &str) -> Option<i32> {
        match name {
            "to_string" => Some(0),
            &_ => None,
        }
    }
}

pub fn parse_integer_literal(p: &mut Parser) -> Option<Node> {
    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<i64>()
        .unwrap();

    let exp = Expression::Integer(Integer { value });

    // if p.lexer.next_is(TokenType::Dot) {
    //     return parse_float_literal(p, exp);
    // }

    Some(Node::Expression(exp))
}

pub fn parse_minus_integer(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<i64>()
        .unwrap();

    Some(Node::Expression(Expression::Integer(Integer {
        value: -value,
    })))
}
