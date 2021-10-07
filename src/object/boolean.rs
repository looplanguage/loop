use crate::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Boolean {
    pub(crate) value: bool,
}

impl ObjectTrait for Boolean {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
