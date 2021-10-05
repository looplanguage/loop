pub mod instructions;
pub mod opcode;
pub mod definition;
mod compile;

use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::instructions::{Instructions, make_instruction};
use crate::compiler::opcode::OpCode;
use crate::object::integer::Integer;
use crate::object::Object;
use crate::parser::expression::Expression;
use crate::parser::program::{Node, Program};
use crate::parser::statement::Statement;

pub struct Compiler {
    pub instructions: Instructions,
    pub constants: Vec<Object>
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

    fn compile_expression(&mut self, expr: Expression) -> Option<String> {
        let err = match expr {
            Expression::Identifier(_) => None,
            Expression::Integer(int) => compile_expression_integer(self, int),
            Expression::Suffix(suffix) => compile_expression_suffix(self, *suffix.clone()),
            Expression::Boolean(_) => None,
            Expression::Function(_) => None,
            Expression::Conditional(_) => None,
        };

        if err.is_some() {
            return err;
        }

        self.emit(OpCode::Pop, vec![]);

        None
    }

    fn compile_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VariableDeclaration(var) => {}
            Statement::Expression(expr) => {
                self.compile_expression(*expr.expression);
            }
            Statement::Block(_) => {}
        }
    }

    fn add_constant(&mut self, obj: Object) -> u16 {
        self.constants.push(obj);

        return (self.constants.len() - 1) as u16
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
        let pos = self.add_instruction(ins);

        pos
    }
}