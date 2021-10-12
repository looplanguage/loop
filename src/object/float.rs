use crate::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Float {
    pub(crate) value: f64,
}

impl ObjectTrait for Float {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
