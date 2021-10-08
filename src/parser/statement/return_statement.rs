use crate::lexer::token::TokenType;
use crate::parser::expression::null::Null;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub expression: Box<crate::parser::expression::Expression>,
}

pub fn parse_return_statement(p: &mut Parser) -> Option<Node> {
    let mut expr = Some(Node::Expression(Expression::Null(Null {})));

    if !p.peek_token_is(TokenType::Semicolon) {
        p.lexer.next_token();

        expr = p.parse_expression(Precedence::Lowest);
    }

    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::Return(ReturnStatement {
            expression: Box::new(exp),
        })));
    }

    None
}
