use self::variable::VariableDeclaration;
use crate::parser::statement::assign::VariableAssign;
use crate::parser::statement::block::Block;
use crate::parser::statement::expression::Expression;

pub mod assign;
pub mod block;
pub mod expression;
pub mod variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
    Block(Block),
    VariableAssign(VariableAssign),
}
