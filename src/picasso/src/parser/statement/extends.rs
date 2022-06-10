use crate::parser::expression::identifier::Identifier;
use crate::parser::program::Node;
use crate::parser::statement::class::{parse_class_statement, ClassItem};
use crate::parser::statement::Statement;
use crate::parser::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct ExtendStatement {
    pub identifier: Identifier,
    pub items: HashMap<String, ClassItem>,
}

pub fn parse_extend_statement(p: &mut Parser) -> Option<Node> {
    // Extend is kind of entirely the same as a class statement, so we can just use that as the
    // parser
    let class_statement = parse_class_statement(p);

    let class = if let Some(Node::Statement(Statement::Class(cls))) = class_statement {
        cls
    } else {
        return None;
    };

    Some(Node::Statement(Statement::Extend(ExtendStatement {
        identifier: Identifier { value: class.name },
        items: class.values,
    })))
}
