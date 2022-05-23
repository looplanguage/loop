use crate::ast::instructions::memory::{CompoundType, Copy, Index, Load, LoadLib, LoadType, Push, Slice, Store};
use crate::ast::instructions::Node;
use crate::lexer::token::Token;
use crate::parser::error::ParseError;
use crate::parser::instruction::function::parse_type_arguments;
use crate::parser::Parser;
use crate::types::{Type, ValueType};

pub fn parse_constant_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    let type_def = parser.parse_type()?;

    let value = if let Type::Compound(name, compound_type) = type_def {
        let mut values: Vec<ValueType> = Vec::new();

        parser.expected(Token::LeftCurly)?;

        while parser.next_token() != Token::RightCurly {
            let constant = parse_constant_instruction(parser)?;

            if let Node::CONSTANT(v) = constant {
                values.push(v);
            }
        }

        ValueType::Compound(name, Box::new(values))
    } else {
        // Second argument is the value
        let next = parser.next_token();

        match next {
            Token::Number(int) => ValueType::Integer(int),
            Token::Boolean(bool) => ValueType::Boolean(bool),
            Token::Float(float) => ValueType::Float(float),
            Token::LeftBracket => parse_array(parser)?,
            Token::String(string) => {
                let mapped: Vec<ValueType> = string.into_iter().map(ValueType::Character).collect();

                ValueType::Array(Box::new(mapped))
            }
            Token::Character(char) => ValueType::Character(char),
            a => {
                return Err(ParseError::UnexpectedToken(Token::Type(Type::INT), a));
            }
        }
    };

    parser.expected(Token::Semicolon)?;

    Ok(Node::CONSTANT(value))
}

fn parse_array(parser: &mut Parser) -> Result<ValueType, ParseError> {
    let mut values: Vec<ValueType> = Vec::new();

    let mut current = parser.next_token();

    while current != Token::RightBracket {
        let cst = parser.parse_node(&current)?;

        if let Node::CONSTANT(c) = cst {
            values.push(c);
        } else {
            return Err(ParseError::Unknown);
        }

        current = parser.next_token();
    }

    Ok(ValueType::Array(Box::new(values)))
}

pub fn parse_index_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let to_index = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::LeftCurly)?;
    let next = &parser.next_token();
    let index = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::INDEX(Index { to_index, index }))
}

pub fn parse_assign_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let to_assign = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::LeftCurly)?;
    let next = &parser.next_token();
    let assign = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::ASSIGN(to_assign, assign))
}

pub fn parse_pop_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let to_pop = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::LeftCurly)?;
    let next = &parser.next_token();
    let pop = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::POP(to_pop, pop))
}

pub fn parse_length_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let current = parser.next_token();
    let ret = Node::LENGTH(Box::new(parser.parse_node(&current)?));

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(ret)
}

pub fn parse_push_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let to_push = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::LeftCurly)?;
    let next = &parser.next_token();
    let item = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::PUSH(Push { to_push, item }))
}

/// `.SLICE { .CONSTANT INT 0; } { .CONSTANT INT 1; } { .CONSTANT INT[] [10,20]; }`
pub fn parse_slice_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let to_slice = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let from = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::LeftCurly)?;

    let next = &parser.next_token();
    let to = Box::new(parser.parse_node(next)?);
    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::SLICE(Slice { to_slice, from, to }))
}

pub fn parse_load_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    // Check if the next argument is the proper one

    let mut node = Load {
        load_type: LoadType::VARIABLE,
        index: 0,
    };

    match parser.next_token() {
        Token::LoadType(tp) => {
            node.load_type = tp.clone();

            if let LoadType::PARAMETER(_) = tp {
                if let Token::Number(unique) = parser.next_token() {
                    node.load_type = LoadType::PARAMETER(unique as u64);
                }
            }
        }
        _ => return Err(ParseError::Unknown),
    }

    let load_type = node.load_type.clone();
    let next_token = parser.next_token();

    parse_load_type(&mut node, next_token, load_type)?;

    parser.expected(Token::Semicolon)?;

    Ok(Node::LOAD(node))
}

fn parse_load_type(
    node: &mut Load,
    current_token: Token,
    load_type: LoadType,
) -> Result<(), ParseError> {
    match current_token {
        Token::Number(int) => match load_type {
            LoadType::VARIABLE => {
                node.index = int as u64;
            }
            LoadType::PARAMETER(_) => {
                node.index = int as u64;
            }
        },
        // Expected an argument, nothing else is possible as index
        _ => return Err(ParseError::Unknown),
    }

    Ok(())
}

/// ```
/// use vinci::ast::AST;
/// use vinci::ast::instructions::memory::Store;
/// use vinci::ast::instructions::Node;
/// use vinci::parse;
/// use vinci::types::ValueType;
/// let mut input = ".STORE 0 { .CONSTANT INT 10; };";
/// let result = parse(input);
///
/// assert_eq!(result, AST {nodes: vec![
///     Node::STORE(Store {
///         index: 0,
///         value: Box::new(Node::CONSTANT(ValueType::Integer(10)))
///     })]
/// });
/// ```
pub fn parse_store_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    let mut node = Store {
        index: 0,
        value: Box::new(Node::CONSTANT(ValueType::Integer(0))),
    };

    if let Token::Number(value) = parser.next_token() {
        node.index = value as u64;
    } else {
        return Err(ParseError::Unknown);
    }

    parser.expected(Token::LeftCurly)?;

    let current = parser.next_token();
    node.value = Box::new(parser.parse_node(&current)?);

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::STORE(node))
}

pub fn parse_loadlib_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    // Parsing of the library path
    let next = parser.next_token();
    let loc = parser.parse_node(&next)?;

    parser.expected(Token::RightCurly)?;

    let t = parser.next_token();
    let mut namespace = String::new();
    if let Token::String(cs) = t {
        for c in cs {
            namespace.push(c);
        }
    }

    parser.expected(Token::Semicolon)?;

    Ok(Node::LOADLIB(LoadLib {
        path: Box::new(loc),
        namespace,
    }))
}

pub fn parse_copy_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    parser.expected(Token::LeftCurly)?;

    // Parsing of the library path
    let next = parser.next_token();
    let obj = parser.parse_node(&next)?;

    parser.expected(Token::RightCurly)?;
    parser.expected(Token::Semicolon)?;

    Ok(Node::COPY(Copy {
        object: Box::new(obj),
    }))
}

pub fn parse_compound_instruction(parser: &mut Parser) -> Result<Node, ParseError> {
    let name = {
        if let Token::String(n) = parser.next_token() {
            let s: String = n.into_iter().collect();
            s
        } else {
            return Err(ParseError::Unknown)
        }
    };

    parser.expected(Token::LeftCurly)?;

    let types = parse_type_arguments(parser)?;
    parser.expected(Token::RightCurly)?;

    parser.expected(Token::Semicolon)?;

    parser.add_custom_type(name.clone(), types.clone())?;

    Ok(Node::COMPOUND(CompoundType {
        name,
        values: Box::new(types)
    }))
}
