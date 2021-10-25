use crate::lib::object::{Object, ObjectTrait};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Array {
    pub(crate) values: Vec<Object>,
}

impl ObjectTrait for Array {
    fn inspect(&self) -> String {
        let mut arr = String::from("[");

        for value in self.values {
            arr.push(format!("{}, ", value.inspect()).parse().unwrap());
        }

        arr.push(']');

        arr
    }
}