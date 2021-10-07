use crate::object::boolean::Boolean;
use crate::object::integer::Integer;

pub mod boolean;
pub mod integer;

pub static TRUE: Object = Object::Boolean(Boolean{ value: true });
pub static FALSE: Object = Object::Boolean(Boolean{ value: false });

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
}

impl Object {
    pub fn inspect(&self) -> String {
        match *self {
            Object::Integer(int) => int.inspect(),
            Object::Boolean(boolean) => boolean.inspect(),
        }
    }
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
