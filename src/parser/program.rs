use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
