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
    // Skips the const
    //p.lexer.next_token();
    println!("{}", p.lexer.get_current_token().unwrap().literal);

    // This "identifier" is for the actual identifier of the constant
    if !p.next_token_is(TokenType::Identifier) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nThe identifiers can contain: letters, numbers and underscores.".to_string();
        p.throw_exception(
            create_token(TokenType::Identifier, "identifier".to_string()),
            Some(message),
        );
    }
    p.lexer.next_token();
    let ident_or_type = p.lexer.get_current_token().unwrap().clone();

    let ident = if p.next_token_is(TokenType::Identifier) {
        let x = Some(p.lexer.current_token.clone().unwrap().clone());
        p.lexer.next_token();
        x
    }
    else {
        None
    };

    // Parsing of the "=" in the declaration
    if !p.next_token_is(TokenType::Colon) {
        let message = "Syntax  -> const <identifier> := <expression>\nExample -> const int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();

        //let message = "Syntax  -> const <datatype> <identifier> := <expression>\nExample -> const int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
        p.throw_exception(
            create_token(TokenType::Assign, "=".to_string()),
            Some(message),
        );
    }
    p.lexer.next_token();

    // Parsing of the "=" in the declaration
    if !p.next_token_is(TokenType::Assign) {
        let message = "Syntax  -> const <datatype> <identifier> = <expression>\nExample -> const int i = 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
        p.throw_exception(
            create_token(TokenType::Assign, "=".to_string()),
            Some(message),
        );
    }
    // Skips the '='
    p.lexer.next_token();
    // Skips the expression
    p.lexer.next_token();
    // Parsing of the expresion, this is the value of the constant
    let expr = p.parse_expression(Precedence::Lowest);
    expr.as_ref()?;
    if ident.is_none() {
        if let Node::Expression(expression) = expr.unwrap() {
            return Some(Node::Statement(Statement::ConstantDeclaration(
                ConstantDeclaration {
                    ident: Identifier {
                        value: ident_or_type.literal,
                    },
                    value: Box::new(expression),
                    data_type: Types::Auto,
                },
            )));
        }

        // This needs to throw a good error
        None
    }
    else {
        if let Node::Expression(expression) = expr.unwrap() {
            return Some(Node::Statement(Statement::ConstantDeclaration(
                ConstantDeclaration {
                    ident: Identifier {
                        value: ident.unwrap().literal,
                    },
                    value: Box::new(expression),
                    data_type: p.parse_type(ident_or_type).unwrap(),
                },
            )));
        }

        // This needs to throw a good error
        None
    }

}
