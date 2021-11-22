use crate::lib::object::array::Array;
use crate::lib::object::boolean::Boolean;
use crate::lib::object::builtin::BuiltinFunction;
use crate::lib::object::float::Float;
use crate::lib::object::function::{CompiledFunction, Function};
use crate::lib::object::hashmap::Hashmap;
use crate::lib::object::integer::Integer;
use crate::lib::object::null::Null;
use crate::lib::object::string::LoopString;
use crate::lib::object::enums::Enums;
use crate::lib::object::ident::Ident;

pub mod array;
pub mod boolean;
pub mod builtin;
pub mod extension_method;
pub mod float;
pub mod function;
pub mod hashmap;
pub mod integer;
pub mod null;
pub mod string;
pub mod enums;
pub mod ident;

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
    Hashmap(Hashmap),
    Enums(Enums),
    Identifier(Ident),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Hashable {
    Integer(Integer),
    String(LoopString),
    Boolean(Boolean),
}

impl Hashable {
    pub fn inspect(&self) -> String {
        match self {
            Hashable::Integer(integer) => integer.inspect(),
            Hashable::String(string) => string.inspect(),
            Hashable::Boolean(boolean) => boolean.inspect(),
        }
    }
}

impl Object {
    pub fn get_hash(&self) -> Option<Hashable> {
        match self {
            Object::Integer(integer) => Some(Hashable::Integer(*integer)),
            Object::String(string) => Some(Hashable::String(string.clone())),
            Object::Boolean(boolean) => Some(Hashable::Boolean(*boolean)),
            _ => None,
        }
    }

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
            Object::Array(array) => array.inspect(),
            Object::Hashmap(hashmap) => hashmap.inspect(),
            Object::Enums(enums) => enums.inspect(),
            Object::Identifier(identifier) => identifier.inspect(),
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
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
