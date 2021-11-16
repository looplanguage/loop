use crate::parser::expression::float::Float;
use crate::parser::expression::integer::Integer;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

pub fn parse_number_literal(p: &mut Parser) -> Option<Node> {
    let value = p.lexer.get_current_token().unwrap().literal.clone();

    if value.parse::<i64>().is_ok() {
        Some(Node::Expression(Expression::Integer(Integer {
            value: value.parse::<i64>().unwrap(),
        })))
    } else if value.parse::<f64>().is_ok() {
        Some(Node::Expression(Expression::Float(Float {
            value: value.parse::<f64>().unwrap(),
        })))
    } else {
        panic!("Error -> Neither a Float of an Integer");
    }
}

pub fn parse_negative_number(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();
    let value = p.lexer.get_current_token().unwrap().literal.clone();

    if value.parse::<i64>().is_ok() {
        Some(Node::Expression(Expression::Integer(Integer {
            value: -value.parse::<i64>().unwrap(),
        })))
    } else if value.parse::<f64>().is_ok() {
        Some(Node::Expression(Expression::Float(Float {
            value: -value.parse::<f64>().unwrap(),
        })))
    } else {
        panic!("Error -> Neither a Float of an Integer");
    }
}
