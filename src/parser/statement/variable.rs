use crate::lexer::token::{create_token, TokenType};
use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::{Expression, Precedence};
use crate::parser::program::Node;
use crate::parser::types::Types;
use crate::parser::Parser;

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Box<Expression>,
    pub data_type: Types,
}

/// A variable declaration looks like this:
///
/// **Syntax:**
/// `<datatype> <identifier> = <expression>`
///
/// **Example:**
/// `int i = 13 + 4`
pub fn parse_variable_declaration(p: &mut Parser, types: Option<Types>) -> Option<Node> {
    // This "identifier" is for the datatype
    // if !p.next_token_is(TokenType::Identifier) {
    //     let message = "Syntax  -> <datatype> <identifier> = <expression>\nExample -> int i = 99\n\nLoop has optional static typing, explanation:\nhttps://looplang.org/docs/concepts/types/primitives".to_string();
    //     p.throw_exception(
    //         create_token(TokenType::Identifier, "var".to_string()),
    //         Some(message),
    //     );
    // }
    // p.lexer.next_token();
    let ident = p.lexer.get_current_token().unwrap().clone();
    let datatype = types.unwrap_or(Types::Auto);
    // Skips the "assign" Token
    p.lexer.next_token();

    // Parsing of the "=" in the declaration
    if !p.next_token_is(TokenType::Assign) {
        let message= if datatype == Types::Auto {
            "Syntax  -> <identifier> := <expression>\nExample -> int i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives".to_string()
        }
        else {
            format!("Syntax  ->  <datatype> <identifier> := <expression>\nExample -> {} i := 99\n\nFor explanation go here:\nhttps://looplang.org/docs/concepts/types/primitives", datatype.transpile())
        };
        p.throw_exception(
            create_token(TokenType::Assign, "=".to_string()),
            Some(message),
        );
    }
    // Skips the: '='
    p.lexer.next_token();
    // Skips the: expression
    p.lexer.next_token();

    let expr = p.parse_expression(Precedence::Lowest);
    expr.as_ref()?;

    if let Node::Expression(exp) = expr.unwrap() {
        return Some(Node::Statement(Statement::VariableDeclaration(
            VariableDeclaration {
                ident: Identifier {
                    value: ident.literal,
                },
                value: Box::new(exp),
                data_type: datatype,
            },
        )));
    }

    // This needs to throw a good error
    println!("{}", 1);
    None
}
