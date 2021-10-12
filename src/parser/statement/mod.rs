use self::variable::VariableDeclaration;
use crate::parser::statement::block::Block;
use crate::parser::statement::expression::Expression;

pub mod block;
pub mod expression;
pub mod variable;
mod test_helper;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
    Block(Block),
}
