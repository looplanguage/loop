use crate::parser::expression::function::parse_call;
use crate::parser::expression::identifier::parse_identifier;
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

    let identifier = parse_identifier(p);
    if let Node::Expression(ident_exp) = identifier.unwrap() {
        p.lexer.next_token();
        let exp = parse_call(p, ident_exp);

        exp.as_ref()?;

        if let Node::Expression(val) = exp.unwrap() {
            return Some(Node::Expression(Expression::Index(Box::from(Index {
                left,
                right: val,
            }))));
        }
    }

    None
}
