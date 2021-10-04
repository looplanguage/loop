use crate::parser::statement::block::Block;
use self::variable::VariableDeclaration;
use crate::parser::statement::expression::Expression;

pub mod expression;
pub mod variable;
pub mod block;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression)
}
