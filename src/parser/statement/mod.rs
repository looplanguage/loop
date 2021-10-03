pub enum Statement {
    VariableDeclaration(VariableDeclaration),
}

use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::Expression;

pub struct VariableDeclaration {
    pub(crate) ident: Identifier,
    pub value: Expression,
}
