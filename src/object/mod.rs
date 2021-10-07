use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::null::Null;

pub mod boolean;
pub mod integer;
pub mod null;

pub static TRUE: Object = Object::Boolean(Boolean { value: true });
pub static FALSE: Object = Object::Boolean(Boolean { value: false });

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
}

impl Object {
    pub fn inspect(&self) -> String {
        match *self {
            Object::Integer(int) => int.inspect(),
            Object::Boolean(boolean) => boolean.inspect(),
            Object::Null(null) => null.inspect(),
        }
    }
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
