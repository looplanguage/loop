use crate::lib::object::array::Array;
use crate::lib::object::boolean::Boolean;
use crate::lib::object::builtin::{BuiltinFunction, EvalResult};
use crate::lib::object::float::Float;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
use crate::lib::object::string::LoopString;

pub mod boolean;
pub mod builtin;
pub mod float;
pub mod function;
pub mod integer;
pub mod null;
pub mod string;
mod array;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    CompiledFunction(CompiledFunction),
    Function(Function),
    Float(Float),
    String(LoopString),
    Builtin(BuiltinFunction),
    Array(Array),
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
            Object::String(string) => string.inspect(),
            Object::Builtin(builtin) => format!("Builtin[{:p}]", builtin),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(bool) => bool.value,
            Object::String(string) => !string.value.is_empty(),
            Object::Null(_) => false,
            _ => true,
        }
    }

    pub fn get_extension_method(
        &self,
        method: i32,
    ) -> Option<Box<dyn Fn(Vec<Object>) -> EvalResult>> {
        match self {
            Object::Integer(integer) => integer.get_extension(method),
            Object::String(string) => string.get_extension(method),
            _ => None,
        }
    }
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
