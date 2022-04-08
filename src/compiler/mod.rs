//! Responsible for transpiling Loop to D
mod compile;
mod modifiers;
mod symbol_table;
mod test;
mod variable_table;

use crate::compiler::compile::expression_array::compile_expression_array;
use crate::compiler::compile::expression_bool::compile_expression_boolean;
use crate::compiler::compile::expression_call::compile_expression_call;
use crate::compiler::compile::expression_conditional::compile_expression_conditional;
use crate::compiler::compile::expression_float::compile_expression_float;
use crate::compiler::compile::expression_function::{compile_expression_function, Function};
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
use crate::compiler::compile::statement_constant_declaration::compile_statement_constant_declaration;
use crate::compiler::compile::statement_export::compile_export_statement;
use crate::compiler::compile::statement_import::compile_import_statement;
use crate::compiler::compile::statement_return::compile_return_statement;
use crate::compiler::compile::statement_variable_assign::compile_statement_variable_assign;
use crate::compiler::compile::statement_variable_declaration::compile_statement_variable_declaration;
use crate::compiler::symbol_table::{Scope, Symbol, SymbolTable};
use crate::compiler::variable_table::{
    build_deeper_variable_scope, build_variable_scope, VariableScope,
};
use crate::lib::exception::compiler::CompilerException;
use crate::lib::exception::compiler_new::CompilerError;
use crate::parser::expression::Expression;
use crate::parser::program::Program;
use crate::parser::statement::block::Block;
use crate::parser::statement::Statement;
use crate::parser::types::Types;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Instance of CompilerResult which contains information on how the compiler handled input
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum CompilerResult {
    // Success can return an optional type if the result was an expression
    Success(Types),
    Optimize,
    Exception(CompilerException),
}

/// The result of the transpiler, which will be passed to the D compiler [crate::lib::util::execute_code]
pub struct DCode {
    pub imports: Vec<String>,
    pub functions: HashMap<String, Function>,
}

/// The compiler itself containing global metadata needed during compilation and methods
pub struct Compiler {
    pub scope_index: i32,
    pub symbol_table: Rc<RefCell<SymbolTable>>,
    pub variable_scope: Rc<RefCell<VariableScope>>,
    pub variable_count: u32,
    pub last_extension_type: Option<Expression>,
    pub location: String,
    pub export_name: String,
    pub prev_location: String,
    pub breaks: Vec<u32>,

    pub imports: Vec<String>,
    pub functions: HashMap<String, Function>,
    pub function_stack: Vec<String>,
    pub current_function: String,
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            scope_index: 0,
            symbol_table: Rc::new(RefCell::new(SymbolTable::new_with_builtins())),
            variable_count: 0,
            variable_scope: Rc::new(RefCell::new(build_variable_scope())),
            last_extension_type: None,
            location: String::new(),
            export_name: String::new(),
            prev_location: String::new(),
            breaks: Vec::new(),

            imports: Vec::new(),
            functions: HashMap::new(),
            function_stack: Vec::new(),
            current_function: String::from("main"),
        }
    }
}

impl Compiler {
    /// Main compilation function which compiles a syntax tree from the parser into D
    ///
    /// # Example
    /// ```
    /// let lexer = lexer::build_lexer("var x = 100");
    /// let mut parser = parser::build_parser(lexer);
    /// let program = parser.parse();
    ///
    /// let compiler = Compiler::default();
    ///
    /// let result = compiler.compile(program);
    ///
    /// if result.is_err() {
    ///     // Handle error
    /// }
    /// ```
    pub fn compile(&mut self, program: Program) -> Result<DCode, CompilerException> {
        // Insert main function
        if self.location.is_empty() {
            let main_function = Function {
                name: "main".to_string(),
                code: String::from("void main() {"),
                parameters: Vec::new(),
                return_type: Types::Void,
            };

            self.functions.insert(String::from("main"), main_function);
        }

        let mut index = 0;
        let length = program.statements.len();
        for statement in program.statements {
            index += 1;

            let mut has_return_value = false;
            let mut is_expression = false;
            let mut added_writeline = false;
            if index == length {
                if let Statement::Expression(_) = statement.clone() {
                    // This is not very good code, the problem is that a user could define a function that returns nothing.
                    // In that case the function will also automatically be printed which will result in an error.
                    // The solution is to recursively walk through the statement to check whether it returns something.
                    // However, currently the print function in stolen from the D STD, which we cannot walk recursively through
                    // because we call it.
                    // TODO: implement a STD with a Loop abstraction so you can walk through the statements
                    has_return_value = !self.is_blacklisted_function_call(statement.clone());
                    is_expression = true;
                    // The last expression always gets printed, but when it is a print functions it doesnt
                    if has_return_value {
                        self.add_import("std".to_string());
                        self.add_to_current_function("writeln(".to_string());
                        added_writeline = true;
                    }
                }
            }

            let err = self.compile_statement(statement, is_expression);

            if index == length {
                if is_expression && has_return_value {
                    self.add_to_current_function(");".to_string());
                } else if !has_return_value {
                    self.add_to_current_function(";".to_string());
                }
            }

            #[allow(clippy::single_match)]
            match err {
                CompilerResult::Exception(exception) => return Result::Err(exception),
                _ => (),
            }
        }

        if self.location.is_empty() {
            self.functions.get_mut("main").unwrap().code.push('}');
        }

        Result::Ok(self.get_d_code())
    }

    /// Defines a new named function and sets it as the compilation scope
    // TODO: Function should have a hashmap of what type each parameter needs to be
    pub fn new_function(&mut self, func: Function) {
        self.function_stack.push(self.current_function.clone());
        self.current_function = func.name.clone();
        self.functions.insert(func.name.clone(), func);
    }

    /// Exits the current function compilation scope
    pub fn exit_function(&mut self) {
        self.current_function = self.function_stack.pop().unwrap();
    }

    /// Adds code to the current function compilation scope
    pub fn add_to_current_function(&mut self, code: String) {
        let func = self.functions.get_mut(&*self.current_function);

        func.unwrap().code.push_str(code.as_str());
    }

    /// Adds an import needed by D This function can be called with the same import as many times
    /// as you would like, it will only add imports if it doesn't already exist
    pub fn add_import(&mut self, import: String) {
        let found = self.imports.iter().find(|&imp| *imp == import);

        if found.is_none() {
            self.imports.push(import);
        }
    }

    /// Loads an internal Loop symbol and adds it to the current function
    pub fn load_symbol(&mut self, symbol: Symbol) {
        match symbol.scope {
            // Parameters in functions
            Scope::Local => {
                self.add_to_current_function(format!("local_{}", symbol.index));
            }
            // Globally defined functions (TODO: should be removed due to transpiling)
            Scope::Global => {}
            // Free "variables" in the scope of closures (D "lambdas") probably unused
            Scope::Free => {
                self.add_to_current_function(format!("local_{}", symbol.index));
            }
            // Builtin symbols, currently this is "std" related symbols and should be replaced by such in the future
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
                    }
                    2 => {
                        self.add_import(String::from("std"));
                        self.add_to_current_function(String::from("writeln"));
                    }
                    _ => {
                        println!("Unknown symbol!");
                    }
                };
            }
        };
    }

    /// Enter a deeper variable scope
    pub fn enter_variable_scope(&mut self) {
        let scope = build_deeper_variable_scope(Option::from(self.variable_scope.clone()));
        self.scope_index += 1;
        self.variable_scope = Rc::new(RefCell::new(scope));
    }

    /// Exit a variable scope and go one shallower
    pub fn exit_variable_scope(&mut self) {
        let outer = self.variable_scope.as_ref().borrow_mut().outer.clone();
        self.scope_index -= 1;
        self.variable_scope = outer.unwrap();
    }

    pub fn get_d_code(&self) -> DCode {
        DCode {
            functions: self.functions.clone(),
            imports: self.imports.clone(),
        }
    }

    /// Compiles the [Expression] [Node](crate::parser::program::Node)
    ///
    /// # Examples
    /// ```
    /// let mut compiler = Compiler::default();
    /// let exp = Expression::Integer(Integer { value: 10 });
    ///
    /// let result = compiler.compile_expression(exp);
    /// ```
    fn compile_expression(&mut self, expr: Expression, is_statement: bool) -> CompilerResult {
        match expr {
            Expression::Identifier(identifier) => compile_expression_identifier(self, identifier),
            Expression::Integer(int) => compile_expression_integer(self, int),
            Expression::Suffix(suffix) => compile_expression_suffix(self, *suffix),
            Expression::Boolean(boolean) => compile_expression_boolean(self, boolean),
            Expression::Function(func) => compile_expression_function(self, func),
            Expression::Conditional(conditional) => {
                compile_expression_conditional(self, *conditional, is_statement)
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

    /// Compiles a loop [Block], this differs from [Compiler::compile_block] in that it wont add curly braces
    fn compile_loop_block(&mut self, block: Block) -> CompilerResult {
        let mut return_type = Types::Void;
        self.enter_variable_scope();

        let mut index = 0;
        for statement in block.statements.clone() {
            index += 1;

            let err = self.compile_statement(statement.clone(), false);

            // If its either a return statement, or the last statement is an expression than that is the return type of this block
            if let Statement::Return(_) = statement {
                if let CompilerResult::Success(_type) = err.clone() {
                    return_type = _type;
                }
            }

            if index == block.statements.len() {
                if let Statement::Expression(_) = statement {
                    if let CompilerResult::Success(_type) = err.clone() {
                        return_type = _type;
                    }
                }
            }

            #[allow(clippy::single_match)]
            match &err {
                CompilerResult::Exception(_exception) => {
                    return CompilerResult::Exception(_exception.clone())
                }
                _ => (),
            }
        }

        self.exit_variable_scope();

        CompilerResult::Success(return_type)
    }

    /// To check whether a statement is the builtin functions: `print` or `println`.
    /// This code is not very good, because it looks for hardcoded function calls,
    /// should recursively walk through statement to check for returns
    fn is_blacklisted_function_call(&self, stat: Statement) -> bool {
        if let Statement::Expression(expr) = stat {
            if let Expression::Call(call) = *expr.expression {
                if let Expression::Identifier(s) = *call.identifier {
                    return s.value == "print" || s.value == "println";
                }
            }
        }

        false
    }

    /// Recursively search through a block to find if it returns anything
    fn block_get_return_type(block: Block) -> bool {
        if !block.statements.is_empty() {
            return match block.statements.last().unwrap() {
                Statement::Expression(exp) => Compiler::should_add_return(*exp.expression.clone()),
                Statement::Return(_) => true,
                _ => false,
            };
        }

        false
    }

    /// Checks an expression if it doesn't already have a return (as expressions always evalaute to a value)
    fn should_add_return(expression: Expression) -> bool {
        // Right now this is a macro, but can be expanded using a matches expression
        !matches!(expression, Expression::Conditional(_))
    }

    /// Compiles a deeper [Block] adding curly braces
    fn compile_block(&mut self, block: Block) -> CompilerResult {
        let mut block_type: Types = Types::Void;
        self.enter_variable_scope();

        self.add_to_current_function("{".to_string());

        let mut index = 0;
        for statement in block.statements.clone() {
            index += 1;

            let err = {
                if let Statement::Expression(exp) = statement.clone() {
                    if Compiler::should_add_return(*exp.expression.clone()) {
                        if index == block.statements.len() {
                            self.add_to_current_function("Variant block_return = ".to_string());
                        }

                        let result = self.compile_statement(statement.clone(), false);

                        if index == block.statements.len() {
                            if let CompilerResult::Success(_type) = &result {
                                block_type = _type.clone();
                            }

                            self.add_to_current_function("return block_return;".to_string());
                        }

                        result
                    } else {
                        let result = self.compile_statement(statement.clone(), false);

                        // Find first "return" as that is the only way to return
                        if let Statement::Return(_) = statement.clone() {
                            if let CompilerResult::Success(_type) = &result {
                                block_type = _type.clone();
                            }
                        }

                        // Or if its the last expression
                        if index == block.statements.len() {
                            if let CompilerResult::Success(_type) = &result {
                                block_type = _type.clone();
                            }
                        }

                        result
                    }
                } else {
                    let result = self.compile_statement(statement.clone(), false);

                    // Find first "return" as that is the only way to return
                    if let Statement::Return(ret) = statement.clone() {
                        if let CompilerResult::Success(_type) = &result {
                            block_type = _type.clone();
                        }
                    }

                    result
                }
            };

            #[allow(clippy::single_match)]
            match &err {
                CompilerResult::Exception(_exception) => {
                    return err;
                }
                _ => (),
            }
        }

        self.add_to_current_function("}".to_string());

        self.exit_variable_scope();

        CompilerResult::Success(block_type)
    }

    /// Compiles the [Statement] [Node](crate::parser::program::Node)
    ///
    /// # Example
    /// ```
    /// let mut compiler = Compiler::default();
    /// let exp = Expression::Integer(Integer { value: 10 });
    /// let stmt = Statement::Expression(exp);
    ///
    /// let result = compiler.compile_statement(stmt);
    /// ```
    fn compile_statement(&mut self, stmt: Statement, no_semicolon: bool) -> CompilerResult {
        let mut expression_statement = false;

        let result = match stmt.clone() {
            Statement::VariableDeclaration(var) => {
                compile_statement_variable_declaration(self, var)
            }
            Statement::ConstantDeclaration(con) => {
                compile_statement_constant_declaration(self, con)
            }
            Statement::Expression(expr) => {
                expression_statement = true;
                self.compile_expression(*expr.expression, true)
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

        let add_semicolon = match stmt {
            Statement::VariableDeclaration(_) => true,
            Statement::ConstantDeclaration(_) => true,
            Statement::Expression(expr) => match *expr.expression {
                Expression::Conditional(_) => !expression_statement,
                Expression::Loop(_) => true,
                Expression::LoopIterator(_) => false,
                Expression::LoopArrayIterator(_) => false,
                Expression::Function(func) => func.name.is_empty(),
                _ => true,
            },
            Statement::Block(_) => false,
            Statement::VariableAssign(_) => true,
            Statement::Return(_) => true,
            Statement::Import(_) => false,
            Statement::Export(_) => true,
            Statement::Break(_) => true,
        };

        if add_semicolon && !no_semicolon {
            self.add_to_current_function(";".to_string());
        }

        result
    }

    /// Throws an [CompilerError](crate::lib::exception::compiler_new::CompilerError;) and exists with code '1'.
    fn throw_exception(&self, message: String, extra_message: Option<String>) {
        let mut err = CompilerError {
            error_message: message,
            extra_message,
        };
        err.throw_exception();
    }
}
