use crate::parser::exception::SyntaxException;
use crate::parser::expression::Precedence;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub expression: Box<crate::parser::expression::Expression>,
}

pub fn parse_expression_statement(p: &mut Parser) -> Result<Node, SyntaxException> {
    let expr = p.parse_expression(Precedence::Lowest)?;

    if let Node::Expression(exp) = expr {
        return Ok(Node::Statement(Statement::Expression(Box::new(
            Expression {
                expression: Box::new(exp),
            },
        ))));
    }

    Err(SyntaxException::Unknown)
}
