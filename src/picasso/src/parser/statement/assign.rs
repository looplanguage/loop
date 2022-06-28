use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::expression::identifier::{Identifier, parse_identifier};
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::Parser;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssign {
    pub ident: Identifier,
    pub value: Box<Expression>,
}

pub fn parse_variable_assignment(p: &mut Parser) -> Result<Node, SyntaxException> {
    let ident = parse_identifier(p)?.into_expression().into_identifier();

    p.lexer
        .next_token_is_and_next_token_result(TokenType::Assign)?;

    p.lexer.next_token();

    let expr = p.parse_expression(Precedence::Lowest)?;

    if let Node::Expression(exp) = expr {
        return Ok(Node::Statement(Statement::VariableAssign(VariableAssign {
            ident: Identifier::new(ident.value, ident.location.line, ident.location.colon),
            value: Box::new(exp),
        })));
    }

    Err(SyntaxException::Unknown)
}
