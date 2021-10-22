use crate::lib::object::ObjectTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct LoopString {
    pub(crate) value: String,
}

impl ObjectTrait for LoopString {
    fn inspect(&self) -> String {
        self.value.clone()
    }
}
