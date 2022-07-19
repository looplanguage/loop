use crate::parser::exception::SyntaxException;
use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location {
    pub line: i32,
    pub colon: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub(crate) value: String,
    pub location: Location,
}

impl Identifier {
    pub fn new(value: String, line: i32, colon: i32) -> Identifier {
        Identifier {
            value,
            location: Location { line, colon },
        }
    }
}

pub fn parse_identifier(p: &mut Parser) -> Result<Node, SyntaxException> {
    Ok(Node::Expression(Expression::Identifier(Identifier::new(
        p.lexer.get_current_token().unwrap().literal.clone(),
        p.lexer.current_line,
        p.lexer.current_col,
    ))))
}
