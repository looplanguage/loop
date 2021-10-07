use crate::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Integer {
    pub(crate) value: i64,
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
