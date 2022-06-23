//! Helper for symbols defined by the user
use crate::compiler::modifiers::Modifiers;
use crate::parser::types::Types;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Variable {
    pub index: u32,
    pub name: String,
    pub _type: Types,
    pub modifiers: Modifiers,
    pub parameter_id: i32,
    pub function_identifier: i32,
}

pub struct VariableScope {
    pub variables: Vec<Rc<RefCell<Variable>>>,
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

impl Variable {
    pub fn transpile(&self) -> String {
        match self._type {
            Types::Compound(_) => format!("class_{}", self.index),
            _ => format!("var_{}", self.index),
        }
    }
}

impl VariableScope {
    pub fn define(
        &mut self,
        index: u32,
        name: String,
        _type: Types,
        modifiers: Modifiers,
        parameter_id: i32,
        function_identifier: i32,
    ) -> Variable {
        println!("Defining: {}", name);
        if name.clone().starts_with("__export_") {
            if let Some(outer) = &self.outer {
                return outer.as_ref().borrow_mut().define(index, name, _type, modifiers, parameter_id, function_identifier);
            }
        }

        self.variables.push(Rc::from(RefCell::from(Variable {
            index,
            name,
            _type: _type.clone(),
            modifiers: modifiers.clone(),
            parameter_id,
            function_identifier,
        })));

        let var = self.variables.last().expect("inserted").as_ref().borrow();

        Variable {
            name: var.name.clone(),
            index: var.index,
            _type,
            modifiers,
            parameter_id,
            function_identifier,
        }
    }

    /// This will get a mutable reference to a variable
    /// **Note**: This will not recursively search the tree upwards, it will **ONLY** look in the
    /// current scope.
    pub fn get_variable_mutable(
        &mut self,
        index: u32,
        name: String,
    ) -> Option<Rc<RefCell<Variable>>> {
        println!("Name: {}", name);
        for rc_variable in &self.variables {
            let variable = rc_variable.as_ref().borrow();
            if variable.name == name && variable.index == index {
                return Some(rc_variable.clone());
            } else {
                if let Some(outer) = &self.outer {
                    return outer.as_ref().borrow_mut().get_variable_mutable(index, name);
                }
            }
        }

        None
    }

    pub fn resolve(&self, name: String) -> Option<Variable> {
        println!("Resolving: {}", name);
        for variable in &self.variables {
            let variable = variable.as_ref().borrow();

            if variable.name == name {
                return Some(Variable {
                    index: variable.index,
                    name,
                    _type: variable._type.clone(),
                    modifiers: variable.modifiers.clone(),
                    parameter_id: variable.parameter_id,
                    function_identifier: variable.function_identifier,
                });
            }
        }

        if let Some(outer) = self.outer.clone() {
            return outer.as_ref().borrow_mut().resolve(name);
        }

        None
    }
}
