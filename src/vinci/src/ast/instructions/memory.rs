use crate::ast::instructions::Node;
use crate::types::Type;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(PartialEq, Eq, Debug, Clone, EnumString)]
pub enum LoadType {
    VARIABLE,
    PARAMETER(u64),
}

#[derive(PartialEq, Debug, Clone)]
pub struct CompoundType {
    pub name: String,
    pub values: Box<Vec<Type>>,
}

impl Display for LoadType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadType::VARIABLE => write!(f, "VARIABLE"),
            LoadType::PARAMETER(_) => write!(f, "PARAMETER"),
        }
    }
}

impl LoadType {
    pub fn find_type(str: &str) -> Option<LoadType> {
        let found = LoadType::from_str(str);

        if let Ok(found) = found {
            Some(found)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Load {
    pub load_type: LoadType,
    pub index: u64,
}

impl Display for Load {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LOAD {} {}", self.load_type, self.index)
    }
}

#[derive(PartialEq, Clone)]
pub struct Store {
    pub index: u64,
    pub value: Box<Node>,
}

impl Display for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "STORE {} {{ {} }}", self.index, self.value)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Index {
    pub to_index: Box<Node>,
    pub index: Box<Node>,
}

impl Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "INDEX {} {}", self.to_index, self.index)
    }
}

#[derive(PartialEq, Clone)]
pub struct Slice {
    pub to_slice: Box<Node>,
    pub from: Box<Node>,
    pub to: Box<Node>,
}

impl Display for Slice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SLICE {} {} {}", self.to_slice, self.from, self.to)
    }
}

#[derive(PartialEq, Clone)]
pub struct Push {
    pub to_push: Box<Node>,
    pub item: Box<Node>,
}

#[derive(PartialEq, Clone)]
pub struct LoadLib {
    pub path: Box<Node>,
    pub namespace: String,
}

impl Display for LoadLib {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LOADLIB {} {}", self.path, self.namespace)
    }
}

impl LoadLib {
    pub fn get_path(&mut self) -> String {
        if let Node::CONSTANT(e) = &*self.path {
            return e.clone().char_arr_to_string();
        }
        panic!("Should be a char array");
    }
}

#[derive(PartialEq, Clone)]
pub struct Copy {
    pub object: Box<Node>,
}

impl Display for Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "COPY {}", self.object)
    }
}

impl Display for Push {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PUSH {} {}", self.to_push, self.item)
    }
}
