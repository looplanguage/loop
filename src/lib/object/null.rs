use crate::lib::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Null {}

impl ObjectTrait for Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }
}
