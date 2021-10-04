use self::variable::VariableDeclaration;
use crate::parser::statement::expression::Expression;

pub mod block;
pub mod expression;
pub mod variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}
