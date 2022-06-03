use crate::parser::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct AssignIndex {
    pub(crate) left: Expression,
    pub(crate) index: Expression,
    pub(crate) value: Expression,
}
