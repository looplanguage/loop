#[derive(Clone, PartialEq, Debug)]
pub enum BaseTypes {
    Integer,
    String,
    Boolean,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Basic(BaseTypes),
    Array(BaseTypes)
}