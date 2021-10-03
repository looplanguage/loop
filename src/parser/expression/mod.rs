use crate::parser::expression::identifier::Identifier;
use crate::parser::expression::integer::Integer;

pub mod identifier;
pub mod integer;

pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),
}
