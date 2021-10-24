use crate::parser::expression::Expression;
use crate::parser::program::Node;
use crate::parser::Parser;

#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub value: f64,
}
