use crate::object::boolean::Boolean;
use crate::object::float::Float;
use crate::object::function::{CompiledFunction, Function};
use crate::object::integer::Integer;
use crate::object::null::Null;

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
