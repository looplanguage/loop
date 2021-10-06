use std::env::var;
use std::ops::Deref;

#[derive(Clone)]
pub struct VariableScope {
    variables: Vec<Variable>,
    outer: Option<Box<VariableScope>>,
}

impl VariableScope {
    pub fn find_variable(&mut self, name: String) -> Option<Variable> {
        for variable in &self.variables {
            if variable.name == name {
                return Some(variable.clone());
            }
        }

        if self.outer.is_some() {
            let mut out = self.outer.clone().unwrap();

            return out.find_variable(name);
        }

        None
    }

    pub fn define_variable(&mut self, name: String) -> u32 {
        let index = self.variables.len() as u32;

        self.variables.push(Variable { index, name });

        index
    }
}

pub fn build_variable_scope(outer: Option<Box<VariableScope>>) -> VariableScope {
    VariableScope {
        variables: vec![],
        outer,
    }
}

#[derive(Clone)]
pub struct Variable {
    pub index: u32,
    pub name: String,
}
