use crate::object::integer::Integer;
use crate::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Float {
    pub(crate) value: f64,
}

impl Float {
    pub fn to_integer(&self) -> Integer {
        Integer {
            value: self.value as i64,
        }
    }
}

impl ObjectTrait for Float {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
