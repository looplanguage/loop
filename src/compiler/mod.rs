mod compile;
pub mod definition;
pub mod instructions;
pub mod opcode;
mod tests;

use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::instructions::{make_instruction, Instructions};
use crate::compiler::opcode::OpCode;
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
}

pub fn build_compiler() -> Compiler {
    Compiler {
        instructions: vec![],
        constants: vec![],
    }
}

impl Compiler {
    pub fn compile(&mut self, program: Program) {
        for statement in program.statements {
            self.compile_statement(statement)
        }
    }

    pub fn get_bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone()
        }
    }

    fn compile_expression(&mut self, expr: Expression) -> Option<String> {
        let err = match expr {
            Expression::Identifier(_) => None,
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

    fn compile_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VariableDeclaration(_var) => {}
            Statement::Expression(expr) => {
                self.compile_expression(*expr.expression);
                self.emit(OpCode::Pop, vec![]);
            }
            Statement::Block(_) => {}
        }
    }

    fn add_constant(&mut self, obj: Object) -> u16 {
        self.constants.push(obj);

        (self.constants.len() - 1) as u16
    }

    fn add_instruction(&mut self, instruction: Vec<u8>) -> usize {
        let position_new_ins = self.instructions.len();

        for ins in instruction {
            self.instructions.push(ins)
        }

        position_new_ins
    }

    fn emit(&mut self, op: OpCode, operands: Vec<u16>) -> usize {
        let ins = make_instruction(op, operands);

        self.add_instruction(ins)
    }
}
