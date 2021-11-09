use crate::lib::object::{Hashable, Object, ObjectTrait};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Hashmap {
    pub values: HashMap<Hashable, Rc<RefCell<Object>>>,
}

impl ObjectTrait for Hashmap {
    fn inspect(&self) -> String {
        let mut _values: Vec<String> = vec![];

        for value in &self.values {
            _values.push(format!(
                "{}: {}",
                value.0.inspect(),
                value.1.borrow().inspect()
            ))
        }

        format!("{{{}}}", _values.join(", "))
    }
}
