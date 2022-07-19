use crate::lexer::token::TokenType;
use crate::parser::exception::SyntaxException;
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
/// `const <datatype> <identifier> := <expression>`
///
/// **Example:**
/// `const i := 13 + 4`
#[allow(clippy::all)]
pub fn parse_constant_declaration(p: &mut Parser) -> Result<Node, SyntaxException> {
    // This "identifier" is for the actual identifier of the constant
    if !p.next_token_is(TokenType::Identifier) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nThe identifiers can contain: letters, numbers and underscores.".to_string();
        return Err(SyntaxException::CustomMessage(
            "expected: Identifier".to_string(),
            Some(message),
        ));
    }

    p.lexer.next_token(); // Skipping the identifier
                          // Depending on if the user has excpliciely typed the type, the current token is a identifier or a type
    let ident_or_type = p.lexer.get_current_token().unwrap().clone();

    // Is user has typed a type, this will be the identifier, otherwise it will be a null
    let ident = if p.next_token_is(TokenType::Identifier) {
        p.lexer.next_token();
        Some(p.lexer.current_token.clone().unwrap())
    } else {
        None
    };

    // Parsing of the ":" in the declaration
    if !p.next_token_is(TokenType::Colon) {
        let message = "Syntax  -> const <identifier> := <expression>\nExample -> const int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();

        //let message = "Syntax  -> const <datatype> <identifier> := <expression>\nExample -> const int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
        return Err(SyntaxException::CustomMessage(
            "expected: ':'".to_string(),
            Some(message),
        ));
    }
    p.lexer.next_token(); // Skipping the ":'

    // Parsing of the "=" in the declaration
    if !p.next_token_is(TokenType::Assign) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();

        return Err(SyntaxException::CustomMessage(
            "expected: '='".to_string(),
            Some(message),
        ));
    }

    p.lexer.next_token(); // Skips the '='
    p.lexer.next_token(); // Skips the expression

    // Parsing of the expresion, this is the value of the constant
    let expr = p.parse_expression(Precedence::Lowest)?;

    if ident.is_none() {
        // Node being created here is for:
        // const i := 3
        if let Node::Expression(expression) = expr {
            return Ok(Node::Statement(Statement::ConstantDeclaration(
                ConstantDeclaration {
                    ident: Identifier::new(ident_or_type.literal, 0, 0),
                    value: Box::new(expression),
                    data_type: Types::Auto,
                },
            )));
        }
    }
    // Node being created here is for:
    // const int i := 3
    if let Node::Expression(expression) = expr {
        return Ok(Node::Statement(Statement::ConstantDeclaration(
            ConstantDeclaration {
                ident: Identifier::new(ident.unwrap().literal, 0, 0),
                value: Box::new(expression),
                data_type: p.parse_type(ident_or_type).unwrap(),
            },
        )));
    }

    // This needs to throw a good error
    Err(SyntaxException::Unknown)
}
