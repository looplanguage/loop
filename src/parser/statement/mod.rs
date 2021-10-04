use self::variable::VariableDeclaration;

pub mod variable;

#[derive(Debug, PartialEq)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
}
