mod compile;
pub mod definition;
pub mod instructions;
pub mod opcode;
mod symbol_table;
mod test;
mod variable_table;

use crate::compiler::compile::expression_array::compile_expression_array;
use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_call::compile_expression_call;
use crate::compiler::compile::expression_conditional::compile_expression_conditional;
use crate::compiler::compile::expression_float::compile_expression_float;
use crate::compiler::compile::expression_function::compile_expression_function;
use crate::compiler::compile::expression_hashmap::compile_expression_hashmap;
use crate::compiler::compile::expression_identifier::compile_expression_identifier;
use crate::compiler::compile::expression_index::{
    compile_expression_assign_index, compile_expression_index,
};
use crate::compiler::compile::expression_integer::compile_expression_integer;
use crate::compiler::compile::expression_loop::{
    compile_loop_array_iterator_expression, compile_loop_expression,
    compile_loop_iterator_expression,
};
use crate::compiler::compile::expression_null::compile_expression_null;
use crate::compiler::compile::expression_string::compile_expression_string;
use crate::compiler::compile::expression_suffix::compile_expression_suffix;
use crate::compiler::compile::statement_break::compile_break_statement;
use crate::compiler::compile::statement_export::compile_export_statement;
use crate::compiler::compile::statement_import::compile_import_statement;
use crate::compiler::compile::statement_return::compile_return_statement;
use crate::compiler::compile::statement_variable_assign::compile_statement_variable_assign;
use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::definition::lookup_op;
use crate::compiler::instructions::{make_instruction, Instructions};
use crate::compiler::opcode::OpCode;
use crate::compiler::symbol_table::{Scope, Symbol, SymbolTable};
use crate::compiler::variable_table::{
    build_deeper_variable_scope, build_variable_scope, VariableScope,
};
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::null::Null;
use crate::lib::object::Object;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::block::Block;
use crate::parser::statement::Statement;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::lib::object::builtin::BUILTINS;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum CompilerResult {
    Success,
    Optimize,
    Exception(CompilerException),
}

pub struct Bytecode {
    pub instructions: Instructions,
    pub constants: Vec<Rc<RefCell<Object>>>,
    pub imports: Vec<String>,
    pub functions: HashMap<String, String>
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
    pub constants: Vec<Rc<RefCell<Object>>>,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
    pub variable_scope: Rc<RefCell<VariableScope>>,
    pub variable_count: u32,
    pub last_extension_type: Option<Expression>,
    pub location: String,
    pub export_name: String,
    pub prev_location: String,
    pub breaks: Vec<u32>,

    pub imports: Vec<String>,
    pub functions: HashMap<String, String>,
    pub function_stack: Vec<String>,
    pub current_function: String
}

pub struct CompilerState {
    constants: Vec<Rc<RefCell<Object>>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    variable_scope: Rc<RefCell<VariableScope>>,
    variable_count: u32,
}

fn build_compiler_internal(state: &CompilerState) -> Compiler {
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
        constants: state.constants.clone(),
        symbol_table: state.symbol_table.clone(),
        variable_count: state.variable_count,
        variable_scope: state.variable_scope.clone(),
        last_extension_type: None,
        location: String::new(),
        export_name: String::new(),
        prev_location: String::new(),
        breaks: Vec::new(),

        imports: Vec::new(),
        functions: HashMap::new(),
        function_stack: Vec::new(),
        current_function: String::from("main")
    }
}

pub fn build_compiler(state: Option<&CompilerState>) -> Compiler {
    if let Some(cmp) = state {
        return build_compiler_internal(cmp);
    }

    build_compiler_internal(&empty_state())
}

fn empty_state() -> CompilerState {
    CompilerState {
        constants: vec![Rc::from(RefCell::from(Object::Null(Null {})))],
        symbol_table: Rc::from(RefCell::new(symbol_table::SymbolTable::new_with_builtins())),
        variable_scope: Rc::new(RefCell::new(build_variable_scope())),
        variable_count: 0,
    }
}

impl Compiler {
    pub fn compile(&mut self, program: Program) -> Result<Bytecode, CompilerException> {
        // Insert main function
        let mut main_function = String::from("void main() {");

        self.functions.insert(String::from("main"), main_function);

        for statement in program.statements {
            let err = self.compile_statement(statement);
            // if let Some(err) = err {
            //     err.emit();
            //
            //     return Result::Err(err);
            // }

            #[allow(clippy::single_match)]
            match err {
                CompilerResult::Exception(exception) => return Result::Err(exception),
                _ => (),
            }
        }

        self.functions.get_mut("main").unwrap().push_str("}");

        Result::Ok(self.get_bytecode())
    }

    pub fn get_state(&self) -> CompilerState {
        CompilerState {
            constants: self.constants.clone(),
            symbol_table: self.symbol_table.clone(),
            variable_count: self.variable_count,
            variable_scope: self.variable_scope.clone(),
        }
    }

    pub fn new_function(&mut self, name: String) {
        self.function_stack.push(self.current_function.clone());
        self.functions.insert(name.clone(), String::new());
        self.current_function = name.clone();
    }

    pub fn exit_function(&mut self) {
        self.current_function = self.function_stack.pop().unwrap();
    }

    pub fn add_to_current_function(&mut self, code: String) {
        let func = self.functions.get_mut(&*self.current_function);

        func.unwrap().push_str(code.as_str());
    }

    pub fn scope(&self) -> &CompilationScope {
        &self.scopes[self.scope_index as usize]
    }

    pub fn scope_mut(&mut self) -> &mut CompilationScope {
        self.scopes[self.scope_index as usize].borrow_mut()
    }

    pub fn add_import(&mut self, import: String) {
        let found = self.imports.iter().find(|&imp| *imp == import);

        if found.is_none() {
            self.imports.push(import);
        }
    }

    pub fn load_symbol(&mut self, symbol: Symbol) {
        match symbol.scope {
            Scope::Local => {self.add_to_current_function(format!("local_{}", symbol.index)); },
            Scope::Global => {self.emit(OpCode::GetVar, vec![symbol.index]); },
            Scope::Free => { self.add_to_current_function(format!("free_{}", symbol.index)); },
            Scope::Builtin => {
                // Temporary
                /*
                    builtin!(len),
                    builtin!(print),
                    builtin!(println),
                    builtin!(format),
                 */
                match symbol.index {
                    1 => {
                        self.add_import(String::from("std"));
                        self.add_to_current_function(String::from("write"));
                    },
                    2 => {
                        self.add_import(String::from("std"));
                        self.add_to_current_function(String::from("writeln"));
                    }
                    _ => {
                        println!("Unknown symbol!");
                    }
                };
            },
        };
    }

    pub fn enter_variable_scope(&mut self) {
        let scope = build_deeper_variable_scope(Option::from(self.variable_scope.clone()));
        self.variable_scope = Rc::new(RefCell::new(scope));
    }

    pub fn exit_variable_scope(&mut self) {
        let outer = self.variable_scope.as_ref().borrow_mut().outer.clone();
        self.variable_scope = outer.unwrap();
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
            functions: self.functions.clone(),
            imports: self.imports.clone()
        }
    }

    fn compile_expression(&mut self, expr: Expression) -> CompilerResult {
        match expr {
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
            Expression::Float(float) => compile_expression_float(self, float),
            Expression::String(string) => compile_expression_string(self, string),
            Expression::Index(index) => compile_expression_index(self, *index),
            Expression::Array(array) => compile_expression_array(self, *array),
            Expression::AssignIndex(assign) => compile_expression_assign_index(self, *assign),
            Expression::Loop(lp) => compile_loop_expression(self, lp),
            Expression::LoopIterator(lp) => compile_loop_iterator_expression(self, lp),
            Expression::LoopArrayIterator(lp) => compile_loop_array_iterator_expression(self, lp),
            Expression::Hashmap(hash) => compile_expression_hashmap(self, hash),
        }
    }

    fn compile_loop_block(&mut self, block: Block) -> CompilerResult {
        self.enter_variable_scope();

        for statement in block.statements {
            let err = self.compile_statement(statement);

            #[allow(clippy::single_match)]
            match &err {
                CompilerResult::Exception(_exception) => return err,
                _ => (),
            }
        }

        self.exit_variable_scope();

        CompilerResult::Success
    }

    fn compile_block(&mut self, block: Block) -> CompilerResult {
        self.enter_variable_scope();

        self.add_to_current_function("{".to_string());

        for statement in block.statements {
            let err = self.compile_statement(statement);

            #[allow(clippy::single_match)]
            match &err {
                CompilerResult::Exception(_exception) => return err,
                _ => (),
            }
        }

        self.add_to_current_function("}".to_string());

        self.exit_variable_scope();

        CompilerResult::Success
    }

    fn compile_statement(&mut self, stmt: Statement) -> CompilerResult {
        let result = match stmt.clone() {
            Statement::VariableDeclaration(var) => {
                compile_statement_variable_declaration(self, var)
            }
            Statement::Expression(expr) => {
                let err = self.compile_expression(*expr.expression);

                self.emit(OpCode::Pop, vec![]);

                err
            }
            Statement::Block(block) => self.compile_block(block),
            Statement::VariableAssign(variable) => {
                compile_statement_variable_assign(self, variable)
            }
            Statement::Return(_return) => compile_return_statement(self, _return),
            Statement::Import(import) => compile_import_statement(self, import),
            Statement::Export(export) => compile_export_statement(self, export),
            Statement::Break(br) => compile_break_statement(self, br),
        };

        let add_semicolon = match stmt.clone() {
            Statement::VariableDeclaration(_) => true,
            Statement::Expression(expr) => {
                match *expr.expression {
                    Expression::Conditional(_) => false,
                    Expression::Loop(_) => false,
                    Expression::LoopIterator(_) => false,
                    Expression::LoopArrayIterator(_) => false,
                    Expression::Function(func) => {
                        if func.name.len() > 0 {
                            false
                        } else {
                            true
                        }
                    }
                    _ => true,
                }
            },
            Statement::Block(_) => false,
            Statement::VariableAssign(_) => true,
            Statement::Return(_) => true,
            Statement::Import(_) => false,
            Statement::Export(_) => false,
            Statement::Break(_) => true,
        };

        if add_semicolon {
            self.add_to_current_function(";".to_string());
        }

        result
    }

    fn add_constant(&mut self, obj: Object) -> u32 {
        self.constants.push(Rc::from(RefCell::from(obj)));

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
