use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Clone, PartialEq, Debug)]
pub struct Export {
    pub expression: Expression,
}

pub fn parse_export_statement(p: &mut Parser) -> Option<Node> {
    p.lexer.next_token();

    let exp = p.parse_expression(Precedence::Lowest);

    if let Some(Node::Expression(expr)) = exp {
        return Some(Node::Statement(Statement::Export(Export {
            expression: expr,
        })));
    }

    None
}
