use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub value: f64,
}

/*pub fn parse_float_literal(p: &mut Parser, left: Expression) -> Option<Node> {
    let integer = match left {
        Expression::Integer(integer) => integer,
        _ => return None,
    };

    p.lexer.next_token();

    let floating_point = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<i64>();

    if floating_point.is_err() {
        p.add_error("incorrect floating point".to_string());
        return None;
    }

    let float: String = format!("{}.{}", integer.value, floating_point.unwrap());
    let value = float.parse::<f64>();

    if value.is_err() {
        return None;
    }

    Some(Node::Expression(Expression::Float(Float {
        value: value.unwrap(),
    })))
}*/

pub fn parse_float_literal(p: &mut Parser) -> Option<Node> {
    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<f64>()
        .unwrap();

    let exp = Expression::Float(Float { value });

    Some(Node::Expression(exp))
}

pub fn parse_minus_float(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let value = p
        .lexer
        .current_token
        .clone()
        .unwrap()
        .literal
        .parse::<f64>()
        .unwrap();

    Some(Node::Expression(Expression::Float(Float {
        value: -value,
    })))
}
