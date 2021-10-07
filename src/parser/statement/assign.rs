use crate::lexer::token::TokenType;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::expression::identifier::Identifier;
use crate::parser::Parser;
use crate::parser::program::Node;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssign {
    pub ident: Identifier,
    pub value: Box<Expression>,
}

pub fn parse_variable_assignment(p: &mut Parser) -> Option<Node> {
    let ident = p.lexer.current_token.clone().unwrap();

    if !p.lexer.next_is(TokenType::Assign) {
        return None;
    }

    p.lexer.next_token();

    let expr = p.parse_expression(Precedence::Lowest);
    expr.as_ref()?;

    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::VariableAssign(
            VariableAssign {
                ident: Identifier {
                    value: ident.literal,
                },
                value: Box::new(exp),
            },
        )));
    }

    None
}
