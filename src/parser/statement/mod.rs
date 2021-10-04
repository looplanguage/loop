use self::variable::VariableDeclaration;
use crate::parser::statement::expression::Expression;

pub mod expression;
pub mod variable;

#[derive(Debug, PartialEq)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}
