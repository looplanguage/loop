use crate::ast::instructions::suffix::{BinaryOperation, Suffix};
use crate::ast::instructions::Node;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

/// Input is .ADD { left; right }
/// ```
/// use vinci::ast::AST;
/// use vinci::ast::instructions::Node;
/// use vinci::ast::instructions::suffix::{BinaryOperation, Suffix};
/// use vinci::parse;
/// use vinci::types::ValueType;
/// let mut input = ".ADD { .CONSTANT INT 10; .CONSTANT INT 20; };";
/// let result = parse(input);
///
/// assert_eq!(result, AST {nodes: vec![
///     Node::SUFFIX(
///         Box::new(Suffix {
///             operation: BinaryOperation::ADD,
///             left: Node::CONSTANT(ValueType::Integer(10)),
///             right: Node::CONSTANT(ValueType::Integer(20))
///         })
///     )]
/// });
/// ```
pub fn parse_math_instruction(
    parser: &mut Parser,
    operation: BinaryOperation,
) -> Result<Node, ParseError> {
    // We expect a curly brace
    parser.expected(Token::LeftCurly)?;

    // Now we expect two nodes, but we use this as a value so we parse it
    let next = parser.next_token();
    let left = parser.parse_node(&next)?;

    let next = parser.next_token();
    let right = parser.parse_node(&next)?;

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::SUFFIX(Box::new(Suffix {
        operation,
        left,
        right,
    })))
}
