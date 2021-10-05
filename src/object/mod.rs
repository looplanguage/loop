use crate::object::integer::Integer;

pub mod integer;

#[derive(Copy, Clone)]
pub enum Object {
    Integer(Integer),
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
