use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::Parser;
use crate::parser::program::Node;

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Expression,
}

pub fn parse_variable_declaration(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::Identifier) {
        return None;
    }

    let ident = p.lexer.current_token.clone().unwrap();

    if !p.lexer.next_is(TokenType::Assign) {
        return None;
    }

    p.lexer.next();

    let expr = p.parse_expression(Precedence::Lowest);
    expr.as_ref()?;

    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::VariableDeclaration(VariableDeclaration {
            ident: Identifier {
                value: ident.literal,
            },
            value: exp,
        })));
    }

    None
}
