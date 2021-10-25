use crate::lib::object::{Object, ObjectTrait};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub(crate) values: Vec<Rc<Object>>,
}

impl ObjectTrait for Array {
    fn inspect(&self) -> String {
        let mut items: Vec<String> = Vec::new();

        for value in &self.values {
            items.push(value.inspect());
        }

        format!("[{}]", items.join(", "))
    }
}
