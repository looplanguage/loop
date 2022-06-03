use crate::lexer::token::Token;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    Unknown,
    // (Expected, Got)
    UnexpectedToken(Token, Token),
    TypeAlreadyExists(String),
    UnknownType(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Unknown => write!(f, "{:?}", self),
            ParseError::UnexpectedToken(expected, got) => {
                write!(f, "expected={:?}. got={:?}", expected, got)
            }
            ParseError::TypeAlreadyExists(name) => {
                write!(f, "Compound type's name already in use. got=\"{}\"", name)
            }
            ParseError::UnknownType(name) => write!(
                f,
                "Type is unknown, did you define it using .COMPOUND? got=\"{}\"",
                name
            ),
        }
    }
}
