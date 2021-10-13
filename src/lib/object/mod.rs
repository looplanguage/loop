use crate::lib::object::boolean::Boolean;
use crate::lib::object::float::Float;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;

pub mod boolean;
pub mod float;
pub mod function;
pub mod integer;
pub mod null;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    CompiledFunction(CompiledFunction),
    Function(Function),
    Float(Float),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(int) => int.inspect(),
            Object::Boolean(boolean) => boolean.inspect(),
            Object::Null(null) => null.inspect(),
            Object::CompiledFunction(func) => func.inspect(),
            Object::Function(func) => func.inspect(),
            Object::Float(float) => float.inspect(),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(value) => value.value,
            Object::Null(_) => false,
            _ => true,
        }
    }
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
