use crate::object::integer::Integer;

pub mod integer;

#[derive(Copy, Clone, Debug)]
pub enum Object {
    Integer(Integer),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self.clone() {
            Object::Integer(int) => int.inspect()
        }
    }
}

pub trait ObjectTrait {
    fn inspect(&self) -> String;
}
