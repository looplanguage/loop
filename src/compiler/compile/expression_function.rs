use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::object::function;
use crate::object::Object::CompiledFunction;
use crate::parser::expression::function::Function;

pub fn compile_expression_function(compiler: &mut Compiler, func: Function) -> Option<String> {
    let num_params = func.parameters.len() as u32;

    compiler.enter_scope();

    for parameter in &func.parameters {
        compiler
            .symbol_table
            .borrow_mut()
            .define(parameter.value.as_str(), 0);
    }

    compiler.compile_function_block(func.body);

    compiler.remove_last(OpCode::Pop);

    let num_locals = compiler.symbol_table.borrow().num_definitions();
    if num_locals > 0xff {
        return Some("too many locals!".to_string());
    }

    let (instructions, free_symbols) = compiler.exit_scope();

    let num_frees = free_symbols.len() as u32;
    if num_frees > 0xff {
        return Some("too many frees!".to_string());
    }

    for free_symbol in free_symbols {
        compiler.load_symbol(free_symbol);
    }

    let compiled_function = CompiledFunction(function::CompiledFunction {
        instructions,
        num_locals: num_locals as u8,
        num_parameters: num_params as u8,
    });

    let const_index = compiler.add_constant(compiled_function);

    compiler.emit(OpCode::Function, vec![const_index, num_frees]);

    None
}
