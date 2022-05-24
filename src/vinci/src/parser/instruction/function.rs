use crate::ast::instructions::function::{Call, Function, LibCall};
use crate::ast::instructions::Node;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use crate::parser::Parser;
use crate::types::Type;
use std::borrow::{Borrow, BorrowMut};

/// ```
/// use vinci::ast::AST;
/// use vinci::ast::instructions::conditional::Conditional;
/// use vinci::ast::instructions::function::Function;
/// use vinci::ast::instructions::memory::{Load, LoadType};
/// use vinci::ast::instructions::Node;
/// use vinci::ast::instructions::suffix::{BinaryOperation, Suffix};
/// use vinci::parse;
/// use vinci::types::{Type, ValueType};
/// let mut input = ".FUNCTION \"named_function\" 0 INT ARGUMENTS {INT;} FREE { } THEN { .LOAD PARAMETER 0 0; };";
/// let result = parse(input);
///
/// assert_eq!(result, AST {nodes: vec![
///     Node::FUNCTION(
///         Box::new(Function {
///             name: "named_function".to_string(),
///             return_type: Type::INT,
///             parameters: vec![Type::INT],
///             body: vec![Node::LOAD(Load {
///                 load_type: LoadType::PARAMETER(0),
///                 index: 0,
///             })],
///             free: vec![],
///             unique_identifier: 0
///         })
///     )]
/// });
/// ```
pub fn parse_function_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    let name = if let Token::String(value) = parser.next_token() {
        value
    } else {
        return Err(ParseError::UnexpectedToken(
            Token::Type(Type::ARRAY(Box::from(Type::CHAR))),
            parser.current_token(),
        ));
    };

    let unique_identifier = if let Token::Number(tp) = parser.next_token() {
        tp
    } else {
        return Err(ParseError::UnexpectedToken(
            Token::Type(Type::INT),
            parser.current_token(),
        ));
    };

    let return_type = parser.parse_type()?;

    parser.expected(Token::Arguments)?;

    parser.expected(Token::LeftCurly)?;

    let parameters: Vec<Type> = parse_type_arguments(parser)?;

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Free)?;
    parser.expected(Token::LeftCurly)?;

    let free: Vec<Type> = parse_type_arguments(parser)?;

    parser.expected(Token::RightCurly)?;

    parser.expected(Token::Then)?;

    parser.expected(Token::LeftCurly)?;

    let body = parser.parse_nodes()?;

    parser.expected(Token::Semicolon)?;

    Ok(Node::FUNCTION(Box::new(Function {
        name: name.into_iter().collect(),
        return_type,
        parameters,
        free,
        body,
        unique_identifier: unique_identifier as i32,
    })))
}

/// ```
/// use vinci::ast::AST;
/// use vinci::ast::instructions::conditional::Conditional;
/// use vinci::ast::instructions::function::{Call, Function};
/// use vinci::ast::instructions::memory::{Load, LoadType};
/// use vinci::ast::instructions::Node;
/// use vinci::ast::instructions::suffix::{BinaryOperation, Suffix};
/// use vinci::parse;
/// use vinci::types::{Type, ValueType};
/// let mut input = ".CALL { .CONSTANT INT 0; } { .CONSTANT INT 10; .CONSTANT INT 30; };";
/// let result = parse(input);
///
/// assert_eq!(result, AST {nodes: vec![
///     Node::CALL(Box::new(
///         Call {
///             call: Node::CONSTANT(ValueType::Integer(0)),
///             arguments: vec![
///                 Node::CONSTANT(ValueType::Integer(10)),
///                 Node::CONSTANT(ValueType::Integer(30)),
///             ]
///         }
///     ))]
/// });
/// ```
pub fn parse_call_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    if let Token::Namespace(namespace) = parser.next_token() {
        parser.expected(Token::LeftCurly)?;

        let arguments = parser.parse_nodes()?;

        parser.expected(Token::Semicolon)?;

        let _x: Vec<&str> = namespace.split("::").collect();

        Ok(Node::LIBCALL(LibCall {
            namespace,
            arguments,
        }))
    } else {
        let next = parser.next_token();

        let function = parser.parse_node(&next)?;

        parser.expected(Token::RightCurly)?;
        parser.expected(Token::LeftCurly)?;

        let arguments = parser.parse_nodes()?;

        parser.expected(Token::Semicolon)?;

        Ok(Node::CALL(Box::new(Call {
            call: function,
            arguments,
        })))
    }
}

pub fn parse_type_arguments(parser: &mut Parser) -> Result<Vec<Type>, ParseError> {
    let mut parameters: Vec<Type> = Vec::new();
    let mut next = parser.lexer.borrow().clone().next().unwrap();

    while next != Token::RightCurly {
        parameters.push(parser.parse_type()?);
        parser.expected(Token::Semicolon)?;
        next = parser.lexer.borrow().clone().next().unwrap();
    }

    Ok(parameters)
}

pub fn parse_return_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let current = parser.next_token();
    let ret = Node::RETURN(Box::new(parser.parse_node(&current)?));

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(ret)
}
