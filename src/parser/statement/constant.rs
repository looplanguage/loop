use crate::lexer::token::{create_token, TokenType};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::statement::Statement;
use crate::parser::types::Types;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantDeclaration {
    pub ident: Identifier,
    pub value: Box<Expression>,
    pub data_type: Types,
}

/// A constant looks like this:
///
/// **Syntax:**
/// `const <datatype> <identifier> = <expression>`
///
/// **Example:**
/// `const int i = 13 + 4`
pub fn parse_constant_declaration(p: &mut Parser) -> Option<Node> {
    // This "identifier" is for the datatype
    if !p.next_token_is(TokenType::Identifier) && !p.next_token_is(TokenType::VariableDeclaration) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nLoop has optional static typing, explanation:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
        p.throw_exception(
            create_token(TokenType::Identifier, "var".to_string()),
            Some(message),
        );
    }
    p.lexer.next_token();
    let datatype = p
        .parse_type(p.lexer.get_current_token().unwrap().clone())
        .unwrap();

    // This "identifier" is for the actual identifier of the constant
    if !p.next_token_is(TokenType::Identifier) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nThe identifiers can contain: letters, numbers and underscores.".to_string();
        p.throw_exception(
            create_token(TokenType::Identifier, "identifier".to_string()),
            Some(message),
        );
    }
    p.lexer.next_token();
    let ident = p.lexer.get_current_token().unwrap().clone();

    // Parsing of the "=" in the declaration
    if !p.next_token_is(TokenType::Assign) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
        p.throw_exception(
            create_token(TokenType::Assign, "=".to_string()),
            Some(message),
        );
    }
    p.lexer.next_token();
    p.lexer.next_token();
    // Parsing of the expresion, this is the value of the constant
    let expr = p.parse_expression(Precedence::Lowest);
    expr.as_ref()?;

    if let Node::Expression(expression) = expr.unwrap() {
        return Some(Node::Statement(Statement::ConstantDeclaration(
            ConstantDeclaration {
                ident: Identifier {
                    value: ident.literal,
                },
                value: Box::new(expression),
                data_type: datatype,
            },
        )));
    }

    // This needs to throw a good error
    None
}
