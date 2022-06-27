use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssign {
    pub ident: Identifier,
    pub value: Box<Expression>,
}

pub fn parse_variable_assignment(p: &mut Parser) -> Result<Node, SyntaxException> {
    let ident = p.lexer.get_current_token().unwrap().clone();

    p.lexer
        .next_token_is_and_next_token_result(TokenType::Assign)?;

    p.lexer.next_token();

    let expr = p.parse_expression(Precedence::Lowest)?;

    if let Node::Expression(exp) = expr {
        return Ok(Node::Statement(Statement::VariableAssign(VariableAssign {
            ident: Identifier {
                value: ident.literal,
            },
            value: Box::new(exp),
        })));
    }

    Err(SyntaxException::Unknown)
}
