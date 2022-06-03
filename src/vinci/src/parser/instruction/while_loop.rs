use crate::ast::instructions::while_loop::While;
use crate::ast::instructions::Node;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;

/// ```
/// use vinci::ast::AST;
/// use vinci::ast::instructions::conditional::Conditional;
/// use vinci::ast::instructions::Node;
/// use vinci::ast::instructions::while_loop::While;
/// use vinci::parse;
/// use vinci::types::ValueType;
/// let mut input = ".WHILE CONDITION { .CONSTANT BOOL true; } THEN { .CONSTANT INT 10; };";
/// let result = parse(input);
///
/// assert_eq!(result, AST {nodes: vec![
///     Node::WHILE(
///         Box::new(While {
///             condition: Node::CONSTANT(ValueType::Boolean(true)),
///             body: vec![Node::CONSTANT(ValueType::Integer(10))],
///         })
///     )]
/// });
/// ```
pub fn parse_while_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::Condition)?;
    parser.expected(Token::LeftCurly)?;

    let current = parser.next_token();
    let condition = parser.parse_node(&current)?;

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Then)?;
    parser.expected(Token::LeftCurly)?;

    let body = parser.parse_nodes()?;

    parser.expected(Token::Semicolon)?;

    Ok(Node::WHILE(Box::new(While { condition, body })))
}
