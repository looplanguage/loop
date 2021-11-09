use crate::lib::object::ObjectTrait;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct LoopString {
    pub(crate) value: String,
}

impl ObjectTrait for LoopString {
    fn inspect(&self) -> String {
        format!("\"{}\"", self.value.clone())
    }
}
