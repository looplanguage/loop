use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
use crate::parser::expression::identifier::{parse_identifier, Identifier};
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::types::Types;
use crate::parser::Parser;

use super::Statement;

/// The struct for a variable declaration, which has an [Identifier], [expression](value) and a [type](Types)
#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Box<Expression>,
    pub data_type: Types,
    // Line, colon
    pub location: (i32, i32),
}

/// A variable declaration looks like this:
///
/// **Syntax:**
/// `<datatype> <identifier> = <expression>`
///
/// **Example:**
/// `int i = 13 + 4`
pub fn parse_variable_declaration(
    p: &mut Parser,
    types: Option<Types>,
) -> Result<Node, SyntaxException> {
    let ident = parse_identifier(p)?.into_expression().into_identifier(); // Identifier of variabele.
    let datatype = types.unwrap_or(Types::Auto);

    // Parsing of the ":" in the declaration
    if !p.next_token_is(TokenType::Colon) {
        let message = "Syntax  -> <identifier> := <expression>\nExample -> i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();

        //let message = "Syntax  -> const <datatype> <identifier> := <expression>\nExample -> const int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
        return Err(SyntaxException::CustomMessage(
            "expected: ':'".to_string(),
            Some(message),
        ));
    }
    p.lexer.next_token(); // Skipping the ":'

    // Parsing of the "=" in the declaration
    if !p.next_token_is(TokenType::Assign) {
        let message = if datatype == Types::Auto {
            "Syntax  -> <identifier> := <expression>\nExample -> int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string()
        } else {
            format!("Syntax  ->  <datatype> <identifier> := <expression>\nExample -> {} i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives", datatype.transpile())
        };
        return Err(SyntaxException::CustomMessage(
            "expected: '='".to_string(),
            Some(message),
        ));
    }

    p.lexer.next_token(); // Skips the: '='
    p.lexer.next_token(); // Skips the: expression

    // Parsing of the expresion, this is the value of the constant
    let expr = p.parse_expression(Precedence::Lowest)?;

    if let Node::Expression(exp) = expr {
        return Ok(Node::Statement(Statement::VariableDeclaration(
            VariableDeclaration {
                ident: Identifier::new(ident.value, ident.location.line, ident.location.colon),
                value: Box::new(exp),
                data_type: datatype,
                location: (p.lexer.current_line, p.lexer.current_col),
            },
        )));
    }

    // This needs to throw a good error
    Err(SyntaxException::Unknown)
}
