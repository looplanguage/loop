use crate::lexer::token::TokenType;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::Parser;

use super::Statement;
use std::borrow::Borrow;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Box<Expression>,
}

pub fn parse_variable_declaration(p: &mut Parser) -> Option<Node> {
    if !p.lexer.next_is(TokenType::Identifier) {
        return None;
    }

    let ident = p.lexer.current_token.clone().unwrap();
    println!("x: {:?}", ident.literal);
    if !p.lexer.next_is(TokenType::Assign) {
        return None;
    }

    p.lexer.next_token();

    let expr = p.parse_expression(Precedence::Lowest);
    expr.as_ref()?;
    println!("Expr: {:?}", Some(expr.borrow()));
    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::VariableDeclaration(
            VariableDeclaration {
                ident: Identifier {
                    value: ident.literal,
                },
                value: Box::new(exp),
            },
        )));
    }

    None
}
