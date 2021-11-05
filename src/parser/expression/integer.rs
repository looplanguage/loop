use crate::parser::expression::string::LoopString;
use crate::parser::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64,
}
