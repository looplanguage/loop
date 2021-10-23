use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Index {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
}

pub fn parse_index_expression(p: &mut Parser, left: Expression) -> Option<Node> {
    p.lexer.next_token();

    let exp = p.parse_expression(Precedence::Lowest);

    exp.as_ref()?;

    if let Node::Expression(val) = exp.unwrap() {
        return Some(Node::Expression(Expression::Index(Box::from(Index {
            left,
            right: val,
        }))));
    }

    None
}
