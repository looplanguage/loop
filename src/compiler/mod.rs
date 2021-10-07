mod compile;
pub mod definition;
pub mod instructions;
pub mod opcode;
mod tests;
mod variable;

use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::instructions::{make_instruction, Instructions};
use crate::compiler::opcode::OpCode;
use crate::compiler::variable::{build_variable_scope, VariableScope};
use crate::object::Object;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::Statement;

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

pub struct Compiler {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
    pub current_variable_scope: VariableScope,
}

pub struct CompilerState {
    constants: Vec<Object>,
    variables: VariableScope,
}

pub fn build_compiler(state: Option<&CompilerState>) -> Compiler {
    if state.is_some() {
        let state_unwrapped = state.unwrap();

        return Compiler {
            instructions: vec![],
            constants: state_unwrapped.constants.clone(),
            current_variable_scope: state_unwrapped.variables.clone(),
        };
    }

    Compiler {
        instructions: vec![],
        constants: vec![],
        current_variable_scope: build_variable_scope(None),
    }
}

impl Compiler {
    pub fn compile(&mut self, program: Program) -> Option<String> {
        for statement in program.statements {
            let err = self.compile_statement(statement);
            if err.is_some() {
                return err;
            }
        }

        None
    }

    pub fn get_state(&self) -> CompilerState {
        CompilerState {
            constants: self.constants.clone(),
            variables: self.current_variable_scope.clone(),
        }
    }

    pub fn get_bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }

    fn compile_expression(&mut self, expr: Expression) -> Option<String> {
        let err = match expr {
            Expression::Identifier(identifier) => {
                let var = self
                    .current_variable_scope
                    .find_variable(identifier.value.clone());

                if var.is_none() {
                    return Some(format!(
                        "variable \"{}\" is not defined in this scope",
                        identifier.value.clone()
                    ));
                }

                let unwrapped = var.unwrap().index;

                self.emit(OpCode::GetVar, vec![unwrapped]);

                None
            }
            Expression::Integer(int) => compile_expression_integer(self, int),
            Expression::Suffix(suffix) => compile_expression_suffix(self, *suffix),
            Expression::Boolean(_) => None,
            Expression::Function(_) => None,
            Expression::Conditional(_) => None,
        };

        if err.is_some() {
            return err;
        }

        None
    }

    fn compile_statement(&mut self, stmt: Statement) -> Option<String> {
        let mut err: Option<String> = None;
        match stmt {
            Statement::VariableDeclaration(var) => {
                let find_variable = self
                    .current_variable_scope
                    .find_variable(var.ident.value.clone());

                if find_variable.is_some() {
                    return Some(format!(
                        "variable \"{}\" is already declared in this scope",
                        find_variable.unwrap().name
                    ));
                }

                err = self.compile_expression(*var.value);

                let id = self.current_variable_scope.define_variable(var.ident.value);

                self.emit(OpCode::SetVar, vec![id]);
            }
            Statement::Expression(expr) => {
                err = self.compile_expression(*expr.expression);

                self.emit(OpCode::Pop, vec![]);
            }
            Statement::Block(_) => {}
            Statement::VariableAssign(variable) => {
                let find_variable = self
                    .current_variable_scope
                    .find_variable(variable.ident.value.clone());

                if find_variable.is_none() {
                    return Some(format!(
                        "variable \"{}\" is not declared in this scope",
                        variable.ident.value
                    ));
                }

                err = self.compile_expression(*variable.value);

                self.emit(OpCode::SetVar, vec![find_variable.unwrap().index]);
            }
        }

        err
    }

    fn add_constant(&mut self, obj: Object) -> u32 {
        self.constants.push(obj);

        (self.constants.len() - 1) as u32
    }

    fn add_instruction(&mut self, instruction: Vec<u8>) -> usize {
        let position_new_ins = self.instructions.len();

        for ins in instruction {
            self.instructions.push(ins)
        }

        position_new_ins
    }

    fn emit(&mut self, op: OpCode, operands: Vec<u32>) -> usize {
        let ins = make_instruction(op, operands);

        self.add_instruction(ins)
    }
}
