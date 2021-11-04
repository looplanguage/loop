use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::object::Object;
use crate::parser::expression::Expression;

pub struct Variable {
    pub index: u32,
    pub name: String,
    pub _type: Expression
}

pub struct VariableScope {
    pub variables: Vec<Variable>,
    pub outer: Option<Rc<RefCell<VariableScope>>>,
}

pub fn build_variable_scope() -> VariableScope {
    VariableScope {
        variables: vec![],
        outer: None,
    }
}
pub fn build_deeper_variable_scope(outer: Option<Rc<RefCell<VariableScope>>>) -> VariableScope {
    VariableScope {
        variables: vec![],
        outer,
    }
}

impl VariableScope {
    pub fn define(&mut self, index: u32, name: String, _type: Expression) -> Variable {
        self.variables.push(Variable { index, name, _type: _type.clone() });

        let var = self.variables.last().expect("inserted");

        Variable {
            name: var.name.clone(),
            index: var.index,
            _type
        }
    }

    pub fn resolve(&mut self, name: String) -> Option<Variable> {
        for variable in &self.variables {
            if variable.name == name {
                return Some(Variable {
                    index: variable.index,
                    name,
                    _type: variable._type.clone()
                });
            }
        }

        if let Some(outer) = self.outer.clone() {
            return outer.as_ref().borrow_mut().resolve(name);
        }

        None
    }
}
