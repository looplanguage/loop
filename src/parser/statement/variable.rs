use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::Parser;

use super::Statement;

pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Expression,
}

pub fn parse_variable_declaration(p: &mut Parser) -> Option<Statement> {
    if !p.lexer.next_is(TokenType::Identifier) {
        return None;
    }

    let ident = p.lexer.current_token.clone().unwrap();

    if !p.lexer.next_is(TokenType::Assign) {
        return None;
    }

    p.lexer.next();

    let expr = p.parse_expression(Precedence::LOWEST);
    if expr.is_none() {
        return None;
    }

    Some(Statement::VariableDeclaration(VariableDeclaration {
        ident: Identifier {
            value: ident.literal,
        },
        value: expr.unwrap(),
    }))
}
