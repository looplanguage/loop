use crate::lib::object::{Object, ObjectTrait};
use std::borrow::BorrowMut;

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub(crate) values: Vec<Object>,
}

impl Array {
    pub fn change_value(&self, index: usize, value: Object) {
        let cur_value = self.values.get(index);

        println!("{:?}", value);

        *cur_value.unwrap().borrow_mut() = &Box::from(value);
    }
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
