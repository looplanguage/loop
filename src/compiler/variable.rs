use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct VariableScope {
    pub(crate) variables: HashMap<String, Variable>,
    pub outer: Option<Box<VariableScope>>,
    pub free: Vec<Variable>,
    pub num_definitions: i32,
}

impl VariableScope {
    pub fn find_variable(&mut self, name: String) -> Option<Variable> {
        for variable in &self.variables {
            if variable.1.name == name {
                return Some(variable.1.clone());
            }
        }

        if self.outer.is_some() {
            let mut out = self.outer.clone().unwrap();

            let variable = out.find_variable(name.clone());

            if variable.is_none() {
                return variable.clone();
            }

            if variable.clone().unwrap().scope == Scope::Global {
                return variable.clone();
            }

            let free = self.define_free(variable.clone().unwrap());

            return Some(free);
        }

        None
    }

    pub fn contains_key(&mut self, name: String) -> i32 {
        let mut i = 0;
        for variable in &self.variables {
            if variable.1.name == name {
                return i;
            }

            i = i + 1;
        }

        -1
    }

    pub fn define_variable(&mut self, name: String) -> Variable {
        let mut variable = Variable {
            name: name.clone(),
            index: self.num_definitions as u32,
            scope: Scope::Global,
        };

        if self.outer.is_none() {
            variable.scope = Scope::Global;
            let new_name = name.clone();

            let idx = self.contains_key(new_name);
            if idx != -1 {
                return self.variables.get(name.clone().as_str()).unwrap().clone();
            }
        } else {
            variable.scope = Scope::Local
        }

        self.variables.insert(variable.name.clone(), variable.clone());
        self.num_definitions += 1;

        return variable.clone();

        /*
        let mut scope = Scope::Local;
        if self.outer.is_none() {
            scope = Scope::Global;
        }

        let var = Variable { index, name, scope };

        self.variables.push(var.clone());

        index*/
    }

    pub fn define_free(&mut self, var: Variable) -> Variable {
        println!("Defining free: {}", var.name);
        let variable = var.clone();

        self.free.push(variable.clone());
        println!("{}", self.free.len());

        let new_variable = Variable {
            name: variable.name.clone(),
            scope: Scope::Free,
            index: (self.free.len() as u32) - 1,
        };

        self.variables.remove(new_variable.name.as_str());
        self.variables.insert(variable.clone().name.clone(), new_variable.clone());

        new_variable
    }
}

pub fn build_variable_scope(outer: Option<Box<VariableScope>>) -> VariableScope {
    VariableScope {
        variables: HashMap::new(),
        outer,
        free: vec![],
        num_definitions: 0,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
    Global,
    Local,
    Free,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub index: u32,
    pub name: String,
    pub scope: Scope,
}
