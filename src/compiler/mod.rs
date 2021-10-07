mod compile;
pub mod definition;
pub mod instructions;
pub mod opcode;
mod tests;
mod variable;

use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_conditional::compile_expression_conditional;
use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_null::compile_expression_null;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::compile::statement_return::compile_return_statement;
use crate::compiler::compile::statement_variable_assign::compile_statement_variable_assign;
use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{make_instruction, Instructions};
use crate::compiler::opcode::OpCode;
use crate::compiler::variable::{build_variable_scope, VariableScope};
use crate::object::null::Null;
use crate::object::Object;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::block::Block;
use crate::parser::statement::Statement;

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

#[derive(Copy, Clone)]
pub struct EmittedInstruction {
    position: i64,
    op: OpCode,
}

pub struct Compiler {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
    pub current_variable_scope: VariableScope,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
    pub return_jumps: Vec<EmittedInstruction>,
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
            last_instruction: EmittedInstruction {
                position: -1,
                op: OpCode::Constant,
            },
            previous_instruction: EmittedInstruction {
                position: -1,
                op: OpCode::Constant,
            },
            return_jumps: vec![],
        };
    }

    Compiler {
        instructions: vec![],
        constants: vec![Object::Null(Null {})],
        current_variable_scope: build_variable_scope(None),
        last_instruction: EmittedInstruction {
            position: -1,
            op: OpCode::Constant,
        },
        previous_instruction: EmittedInstruction {
            position: -1,
            op: OpCode::Constant,
        },
        return_jumps: vec![],
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
            Expression::Conditional(conditional) => {
                compile_expression_conditional(self, *conditional)
            }
            Expression::Null(_) => compile_expression_null(self),
        };

        if err.is_some() {
            return err;
        }

        None
    }

    fn enter_variable_scope(&mut self) {
        self.current_variable_scope = VariableScope {
            variables: vec![],
            outer: Option::from(Box::new(self.current_variable_scope.clone())),
        };
    }

    fn exit_variable_scope(&mut self) {
        self.current_variable_scope = *self.current_variable_scope.outer.clone().unwrap();
    }

    fn compile_block(&mut self, block: Block) -> Option<String> {
        self.enter_variable_scope();

        for statement in block.statements {
            let err = self.compile_statement(statement);
            if err.is_some() {
                return err;
            }
        }

        self.exit_variable_scope();

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
            Statement::Return(_return) => {
                err = compile_return_statement(self, _return);
            }
        }

        err
    }

    fn add_constant(&mut self, obj: Object) -> u32 {
        self.constants.push(obj);

        (self.constants.len() - 1) as u32
    }

    fn last_is(&mut self, op: OpCode) -> bool {
        if self.last_instruction.op == op {
            return true;
        }

        false
    }

    fn remove_last(&mut self, op: OpCode) -> bool {
        if self.last_instruction.op == op {
            let old_ins = self.instructions.clone();
            let ins = &old_ins[..self.last_instruction.position as usize];

            self.instructions = Instructions::from(ins);

            return true;
        }

        false
    }

    fn add_instruction(&mut self, instruction: Vec<u8>) -> usize {
        let position_new_ins = self.instructions.len();

        for ins in instruction {
            self.instructions.push(ins)
        }

        position_new_ins
    }

    fn replace_instruction(&mut self, pos: u32, instruction: Vec<u8>) {
        let instructions: &mut Instructions = self.instructions.as_mut();

        let mut i = 0;
        while i < instruction.len() {
            instructions[pos as usize + i] = instruction[i];
            i += 1;
        }
    }

    fn change_operand(&mut self, pos: u32, operands: Vec<u32>) {
        let op = self.instructions[pos as usize];
        let opc = lookup_op(op);

        if let Some(opcode) = opc {
            let new_instruction = make_instruction(opcode, operands);
            self.replace_instruction(pos, new_instruction)
        }
    }

    fn emit(&mut self, op: OpCode, operands: Vec<u32>) -> usize {
        let ins = make_instruction(op, operands);

        self.previous_instruction = self.last_instruction;

        let pos = self.add_instruction(ins.clone());

        self.last_instruction = EmittedInstruction {
            position: pos as i64,
            op,
        };

        pos
    }
}
