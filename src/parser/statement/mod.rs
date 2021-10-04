use crate::parser::statement::expression::Expression;
use self::variable::VariableDeclaration;

pub mod variable;
pub mod expression;

#[derive(Debug, PartialEq)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression)
}
