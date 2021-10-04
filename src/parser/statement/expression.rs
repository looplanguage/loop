use crate::parser::expression::Precedence;
use crate::parser::statement::Statement;
use crate::parser::Parser;
use crate::parser::program::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub expression: crate::parser::expression::Expression,
}

pub fn parse_expression_statement(p: &mut Parser) -> Option<Node> {
    let expr = p.parse_expression(Precedence::Lowest);

    expr.as_ref()?;

    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::Expression(Expression {
            expression: exp,
        })));
    }

    None
}
