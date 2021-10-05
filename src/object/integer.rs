use crate::object::ObjectTrait;

#[derive(Copy, Clone, Debug)]
pub struct Integer {
    pub(crate) value: i32,
}

impl ObjectTrait for Integer {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
