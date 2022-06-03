use crate::ast::instructions::conditional::Conditional;
use crate::ast::instructions::Node;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

/// ```
/// use vinci::ast::AST;
/// use vinci::ast::instructions::conditional::Conditional;
/// use vinci::ast::instructions::Node;
/// use vinci::ast::instructions::suffix::{BinaryOperation, Suffix};
/// use vinci::parse;
/// use vinci::types::ValueType;
/// let mut input = ".IF CONDITION { .CONSTANT BOOL true; } THEN { .CONSTANT INT 10; } ELSE { .CONSTANT INT 20; };";
/// let result = parse(input);
///
/// assert_eq!(result, AST {nodes: vec![
///     Node::CONDITIONAL(
///         Box::new(Conditional {
///             condition: Node::CONSTANT(ValueType::Boolean(true)),
///             body: vec![Node::CONSTANT(ValueType::Integer(10))],
///             alternative: vec![Node::CONSTANT(ValueType::Integer(20))]
///         })
///     )]
/// });
/// ```
pub fn parse_conditional_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    let mut alternative: Vec<Node> = Vec::new();

    parser.expected(Token::Condition)?;
    parser.expected(Token::LeftCurly)?;

    let current = parser.next_token();
    let condition = parser.parse_node(&current)?;

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Then)?;
    parser.expected(Token::LeftCurly)?;

    let body = parser.parse_nodes()?;

    if parser.expected_maybe(Token::Else) {
        parser.expected(Token::LeftCurly)?;
        alternative = parser.parse_nodes()?;
    }

    parser.expected(Token::Semicolon)?;

    Ok(Node::CONDITIONAL(Box::new(Conditional {
        condition,
        body,
        alternative,
    })))
}

pub fn parse_and_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    // We expect a curly brace
    parser.expected(Token::LeftCurly)?;

    // Now we expect two nodes, but we use this as a value so we parse it
    let next = parser.next_token();
    let left = Box::new(parser.parse_node(&next)?);

    let next = parser.next_token();
    let right = Box::new(parser.parse_node(&next)?);

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::AND(left, right))
}

pub fn parse_or_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    // We expect a curly brace
    parser.expected(Token::LeftCurly)?;

    // Now we expect two nodes, but we use this as a value so we parse it
    let next = parser.next_token();
    let left = Box::new(parser.parse_node(&next)?);

    let next = parser.next_token();
    let right = Box::new(parser.parse_node(&next)?);

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::OR(left, right))
}
