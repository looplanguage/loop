use crate::object::boolean::Boolean;
use crate::object::integer::Integer;

pub mod boolean;
pub mod integer;

#[derive(Copy, Clone, Debug)]
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
