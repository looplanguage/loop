use crate::lexer;
use crate::lexer::token::TokenType;
use crate::parser::Parser;
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::Expression;

use super::Statement;

pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Expression,
}

pub fn parse_variable_declaration(p: &mut Parser) -> Option<Statement> {
  let ident = p.lexer.next();
  if ident.token != TokenType::Identifier {
      return None;
  }

  if !p.lexer.next_is(TokenType::Assign) {
      return None;
  }

  let expr = p.parse_expression();
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
