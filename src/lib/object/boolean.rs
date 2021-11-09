use crate::lib::object::ObjectTrait;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Boolean {
    pub(crate) value: bool,
}

impl ObjectTrait for Boolean {
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
