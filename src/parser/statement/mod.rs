use crate::parser::statement::block::Block;
use crate::parser::statement::expression::Expression;

use self::variable::VariableDeclaration;

pub mod block;
pub mod expression;
pub mod variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
    Block(Block),
}
