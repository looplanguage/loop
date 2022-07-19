use crate::parser::exception::SyntaxException;
use crate::parser::expression::identifier::Identifier;
use crate::parser::program::Node;
use crate::parser::statement::class::{parse_class_statement, ClassField};
use crate::parser::statement::Statement;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct ExtendStatement {
    pub identifier: Identifier,
    pub items: Vec<ClassField>,
}

pub fn parse_extend_statement(p: &mut Parser) -> Result<Node, SyntaxException> {
    // Extend is kind of entirely the same as a class statement, so we can just use that as the
    // parser
    let class_statement = parse_class_statement(p)?;

    let class = if let Node::Statement(Statement::Class(cls)) = class_statement {
        cls
    } else {
        unreachable!()
    };

    Ok(Node::Statement(Statement::Extend(ExtendStatement {
        identifier: Identifier::new(class.name, 0, 0),
        items: class.values,
    })))
}
