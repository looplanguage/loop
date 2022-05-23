use crate::ast::instructions::Node;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone)]
pub enum BinaryOperation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    POWER,
    GREATERTHAN,
    EQUALS,
    NOTEQUALS,
    MODULO,
}

#[derive(PartialEq, Clone)]
pub struct Suffix {
    pub operation: BinaryOperation,
    pub left: Node,
    pub right: Node,
}

impl Display for Suffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.operation {
            BinaryOperation::ADD => write!(f, "({} + {})", self.left, self.right),
            BinaryOperation::SUBTRACT => write!(f, "({} - {})", self.left, self.right),
            BinaryOperation::MULTIPLY => write!(f, "({} * {})", self.left, self.right),
            BinaryOperation::DIVIDE => write!(f, "({} / {})", self.left, self.right),
            BinaryOperation::POWER => write!(f, "({} ^ {})", self.left, self.right),
            BinaryOperation::GREATERTHAN => write!(f, "({} > {})", self.left, self.right),
            BinaryOperation::EQUALS => write!(f, "({} == {})", self.left, self.right),
            BinaryOperation::NOTEQUALS => write!(f, "({} != {})", self.left, self.right),
            BinaryOperation::MODULO => write!(f, "({} % {})", self.left, self.right),
        }
    }
}
