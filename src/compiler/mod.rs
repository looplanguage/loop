mod compile;
pub mod definition;
pub mod instructions;
pub mod opcode;
mod tests;
mod variable;

use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::compile::statement_variable_assign::compile_statement_variable_assign;
use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
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
    if let Some(cmp) = state {
        return Compiler {
            instructions: vec![],
            constants: cmp.constants.clone(),
            current_variable_scope: cmp.variables.clone(),
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
            Expression::Identifier(identifier) => compile_expression_identifier(self, identifier),
            Expression::Integer(int) => compile_expression_integer(self, int),
            Expression::Suffix(suffix) => compile_expression_suffix(self, *suffix),
            Expression::Boolean(boolean) => compile_expression_boolean(self, boolean),
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
                err = compile_statement_variable_declaration(self, var);
            }
            Statement::Expression(expr) => {
                err = self.compile_expression(*expr.expression);

                self.emit(OpCode::Pop, vec![]);
            }
            Statement::Block(_) => {}
            Statement::VariableAssign(variable) => {
                err = compile_statement_variable_assign(self, variable);
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
