use crate::parser::expression::Expression;
use crate::parser::statement::Statement;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}

impl Node {
    /// Casts to and panics if it can't
    pub fn into_expression(self) -> Expression {
        if let Node::Expression(exp) = self {
            exp
        } else {
            panic!("Unable to cast into expression!")
        }
    }

    /// Casts to and panics if it can't
    pub fn into_statement(self) -> Statement {
        if let Node::Statement(stmt) = self {
            stmt
        } else {
            panic!("Unable to cast into expression!")
        }
    }
}

/// The final program is just a list of statements
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
