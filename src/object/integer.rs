use crate::object::ObjectTrait;

pub struct Integer {
    pub(crate) value: i32
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}