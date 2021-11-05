use crate::lib::object::{Object, ObjectTrait};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub(crate) values: Vec<Rc<RefCell<Object>>>,
}

impl ObjectTrait for Array {
    fn inspect(&self) -> String {
        let mut items: Vec<String> = Vec::new();

        for value in &self.values {
            items.push(value.borrow().inspect());
        }

        format!("[{}]", items.join(", "))
    }
}
