use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::function;
use crate::lib::object::Object::CompiledFunction;
use crate::parser::expression::function::Function;

pub fn compile_expression_function(
    compiler: &mut Compiler,
    func: Function,
) -> Option<CompilerException> {
    let num_params = func.parameters.len() as u32;

    compiler.enter_scope();

    for parameter in &func.parameters {
        compiler
            .symbol_table
            .borrow_mut()
            .define(parameter.value.as_str(), 0);
    }

    let err = compiler.compile_block(func.body.clone());
    if err.is_some() {
        return err;
    }

    compiler.remove_last(OpCode::Pop);

    let num_locals = compiler.symbol_table.borrow().num_definitions();
    if num_locals > 0xff {
        return Some(CompilerException::TooManyLocals);
    }

    let (instructions, free_symbols) = compiler.exit_scope();

    let num_frees = free_symbols.len() as u32;
    if num_frees > 0xff {
        return Some(CompilerException::TooManyFrees);
    }

    for free_symbol in free_symbols {
        compiler.load_symbol(free_symbol);
    }

    let mut parsed_function = None;

    if compiler.jit_enabled {
        parsed_function = Some(func);
    }

    let compiled_function = CompiledFunction(function::CompiledFunction {
        instructions,
        num_locals: num_locals as u8,
        num_parameters: num_params as u8,
        parsed_function
    });

    let const_index = compiler.add_constant(compiled_function);

    compiler.emit(OpCode::Function, vec![const_index, num_frees]);

    None
}
