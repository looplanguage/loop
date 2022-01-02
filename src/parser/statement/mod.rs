use self::variable::VariableDeclaration;
use crate::parser::statement::assign::VariableAssign;
use crate::parser::statement::block::Block;
use crate::parser::statement::break_statement::BreakStatement;
use crate::parser::statement::export::Export;
use crate::parser::statement::expression::Expression;
use crate::parser::statement::import::Import;
use crate::parser::statement::return_statement::ReturnStatement;
pub mod assign;
pub mod block;
pub mod break_statement;
pub mod export;
pub mod expression;
pub mod import;
pub mod return_statement;
pub mod variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Box<Expression>),
    Block(Block),
    VariableAssign(VariableAssign),
    Return(ReturnStatement),
    Import(Import),
    Export(Export),
    Break(BreakStatement),
}
