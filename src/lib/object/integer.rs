use crate::lib::object::float::Float;
use crate::lib::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Integer {
    pub(crate) value: i64,
}

impl Integer {
    pub fn to_float(self) -> Float {
        Float {
            value: self.value as f64,
        }
    }
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
