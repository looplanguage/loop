use crate::lib::object::{Object, ObjectTrait};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Enums {
    pub(crate) identifiers: Vec<Rc<RefCell<Object>>>,
}

impl ObjectTrait for Enums {
    fn inspect(&self) -> String {
        let mut items: Vec<String> = Vec::new();

        for value in &self.identifiers {
            items.push(value.borrow().inspect());
        }

        format!("[{}]", items.join(", "))
    }
}
