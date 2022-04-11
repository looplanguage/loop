use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

/// The final program is just a list of statements
pub struct Program {
    pub statements: Vec<Statement>,
}
