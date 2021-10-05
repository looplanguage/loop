use crate::parser::expression::Precedence;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub expression: Box<crate::parser::expression::Expression>,
}

pub fn parse_expression_statement(p: &mut Parser) -> Option<Node> {
    let expr = p.parse_expression(Precedence::Lowest);

    expr.as_ref()?;

    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::Expression(Box::new(Expression {
            expression: Box::new(exp),
        }))));
    }

    None
}
