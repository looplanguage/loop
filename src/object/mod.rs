use crate::object::boolean::Boolean;
use crate::object::function::{CompiledFunction, Function};
use crate::object::integer::Integer;
use crate::object::null::Null;

pub mod boolean;
pub mod function;
pub mod integer;
pub mod null;

pub static TRUE: Object = Object::Boolean(Boolean { value: true });
pub static FALSE: Object = Object::Boolean(Boolean { value: false });

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    CompiledFunction(CompiledFunction),
    Function(Function),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(int) => int.inspect(),
            Object::Boolean(boolean) => boolean.inspect(),
            Object::Null(null) => null.inspect(),
            Object::CompiledFunction(func) => func.inspect(),
            Object::Function(func) => func.inspect(),
        }
    }
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
