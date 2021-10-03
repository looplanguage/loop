pub enum Statement {
    VariableDeclaration(VariableDeclaration),
}

use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::Expression;

pub struct VariableDeclaration {
    pub ident: Identifier,
    pub value: Expression,
}
