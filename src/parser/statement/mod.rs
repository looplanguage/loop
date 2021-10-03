use self::variable::VariableDeclaration;

pub mod variable;

pub enum Statement {
    VariableDeclaration(VariableDeclaration),
}
