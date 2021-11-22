use crate::lib::object::ObjectTrait;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Ident {
    pub(crate) value: String,
}

impl ObjectTrait for Ident {
    fn inspect(&self) -> String {
        format!("\"{}\"", self.value.clone())
    }
}
