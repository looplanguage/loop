mod compile;
pub mod definition;
pub mod instructions;
pub mod opcode;
mod symbol_table;
mod tests;

use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_call::compile_expression_call;
use crate::compiler::compile::expression_conditional::compile_expression_conditional;
use crate::compiler::compile::expression_function::compile_expression_function;
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
use crate::compiler::symbol_table::{Scope, Symbol, SymbolTable};
use crate::object::null::Null;
use crate::object::Object;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::block::Block;
use crate::parser::statement::Statement;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Object>,
}

#[derive(Copy, Clone)]
pub struct EmittedInstruction {
    position: i64,
    op: OpCode,
}

pub struct CompilationScope {
    pub instructions: Instructions,
    pub last_instruction: EmittedInstruction,
    pub previous_instruction: EmittedInstruction,
}

pub struct Compiler {
    pub scopes: Vec<CompilationScope>,
    pub scope_index: i32,
    pub constants: Vec<Object>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
    pub variable_count: u32,
}

pub struct CompilerState {
    constants: Vec<Object>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    variable_count: u32,
}

pub fn build_compiler(state: Option<&CompilerState>) -> Compiler {
    if let Some(cmp) = state {
        return Compiler {
            scopes: vec![CompilationScope {
                instructions: vec![],
                last_instruction: EmittedInstruction {
                    position: -1,
                    op: OpCode::Constant,
                },
                previous_instruction: EmittedInstruction {
                    position: -1,
                    op: OpCode::Constant,
                },
            }],
            scope_index: 0,
            constants: cmp.constants.clone(),
            symbol_table: cmp.symbol_table.clone(),
            variable_count: cmp.variable_count,
        };
    }

    Compiler {
        scopes: vec![CompilationScope {
            instructions: vec![],
            last_instruction: EmittedInstruction {
                position: -1,
                op: OpCode::Constant,
            },
            previous_instruction: EmittedInstruction {
                position: -1,
                op: OpCode::Constant,
            },
        }],
        scope_index: 0,
        constants: vec![Object::Null(Null {})],
        symbol_table: Rc::new(RefCell::new(SymbolTable::new_with_builtins())),
        variable_count: 0,
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
            symbol_table: self.symbol_table.clone(),
            variable_count: self.variable_count,
        }
    }

    pub fn scope(&self) -> &CompilationScope {
        &self.scopes[self.scope_index as usize]
    }

    pub fn scope_mut(&mut self) -> &mut CompilationScope {
        self.scopes[self.scope_index as usize].borrow_mut()
    }

    pub fn load_symbol(&mut self, symbol: Symbol) {
        match symbol.scope {
            Scope::Local => self.emit(OpCode::GetLocal, vec![symbol.index]),
            Scope::Global => self.emit(OpCode::GetVar, vec![symbol.index]),
            Scope::Free => self.emit(OpCode::GetFree, vec![symbol.index]),
            Scope::Builtin => 0 as usize,
        };
    }

    pub fn enter_scope(&mut self) {
        let scope = CompilationScope {
            instructions: vec![],
            last_instruction: EmittedInstruction {
                position: -1,
                op: OpCode::Constant,
            },
            previous_instruction: EmittedInstruction {
                position: -1,
                op: OpCode::Constant,
            },
        };

        self.scopes.push(scope);
        self.scope_index += 1;

        self.symbol_table.as_ref().borrow_mut().push();
    }

    pub fn exit_scope(&mut self) -> (Instructions, Vec<Symbol>) {
        let current_scope = self.scope();
        let ins = current_scope.instructions.clone();

        self.scopes.pop();
        self.scope_index -= 1;

        let free = self.symbol_table.as_ref().borrow_mut().pop();

        (ins, free)
    }

    pub fn get_bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: self.scope().instructions.clone(),
            constants: self.constants.clone(),
        }
    }

    fn compile_expression(&mut self, expr: Expression) -> Option<String> {
        let err = match expr {
            Expression::Identifier(identifier) => compile_expression_identifier(self, identifier),
            Expression::Integer(int) => compile_expression_integer(self, int),
            Expression::Suffix(suffix) => compile_expression_suffix(self, *suffix),
            Expression::Boolean(boolean) => compile_expression_boolean(self, boolean),
            Expression::Function(func) => compile_expression_function(self, func),
            Expression::Conditional(conditional) => {
                compile_expression_conditional(self, *conditional)
            }
            Expression::Null(_) => compile_expression_null(self),
            Expression::Call(call) => compile_expression_call(self, call),
        };

        if err.is_some() {
            return err;
        }

        None
    }

    fn compile_block(&mut self, block: Block) -> Option<String> {
        if block.statements.is_empty() {
            compile_expression_null(self);
        }

        for statement in block.statements {
            let err = self.compile_statement(statement);
            if err.is_some() {
                return err;
            }
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
        if self.scope().last_instruction.op == op {
            return true;
        }

        false
    }

    fn remove_last(&mut self, op: OpCode) -> bool {
        if self.scope().last_instruction.op == op {
            let old_ins = self.scope().instructions.clone();
            let ins = &old_ins[..self.scope().last_instruction.position as usize];
            let prev_ins = self.scope().previous_instruction;

            let mut s = self.scope_mut();
            s.instructions = Instructions::from(ins);

            s.last_instruction = prev_ins;

            return true;
        }

        false
    }

    fn add_instruction(&mut self, instruction: Vec<u8>) -> usize {
        let position_new_ins = self.scope().instructions.len();

        for ins in instruction {
            let scope = self.scope_mut();
            scope.instructions.push(ins)
        }

        position_new_ins
    }

    fn replace_instruction(&mut self, pos: u32, instruction: Vec<u8>) {
        let scope = self.scope_mut();
        let instructions: &mut Instructions = scope.instructions.as_mut();

        let mut i = 0;
        while i < instruction.len() {
            instructions[pos as usize + i] = instruction[i];
            i += 1;
        }
    }

    fn change_operand(&mut self, pos: u32, operands: Vec<u32>) {
        let op = self.scope().instructions[pos as usize];
        let opc = lookup_op(op);

        if let Some(opcode) = opc {
            let new_instruction = make_instruction(opcode, operands);
            self.replace_instruction(pos, new_instruction)
        }
    }

    fn emit(&mut self, op: OpCode, operands: Vec<u32>) -> usize {
        let ins = make_instruction(op, operands);

        let pos = self.add_instruction(ins);

        let scope = self.scope_mut();
        scope.previous_instruction = scope.last_instruction;
        scope.last_instruction = EmittedInstruction {
            position: pos as i64,
            op,
        };

        pos
    }
}
