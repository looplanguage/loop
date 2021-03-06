use crate::parser::exception::SyntaxException;
use crate::parser::expression::null::Null;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct BreakStatement {
    pub expression: Box<crate::parser::expression::Expression>,
}

pub fn parse_break_statement(_: &mut Parser) -> Result<Node, SyntaxException> {
    let expr = Node::Expression(Expression::Null(Null {}));

    // TODO: Allow breaking with a value
    /*
    if !p.peek_token_is(TokenType::Semicolon) {
        p.lexer.next_token();

        expr = p.parse_expression(Precedence::Lowest);
    }*/

    if let Node::Expression(exp) = expr {
        return Ok(Node::Statement(Statement::Break(BreakStatement {
            expression: Box::new(exp),
        })));
    }

    Err(SyntaxException::Unknown)
}
