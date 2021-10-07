use crate::parser::statement::assign::VariableAssign;
use self::variable::VariableDeclaration;
use crate::parser::statement::block::Block;
use crate::parser::statement::expression::Expression;

pub mod block;
pub mod expression;
pub mod variable;
pub mod assign;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
    Block(Block),
    VariableAssign(VariableAssign)
}
