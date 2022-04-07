use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> CompilerResult {
    let symbol = compiler
        .symbol_table
        .borrow_mut()
        .resolve(format!("{}{}", compiler.location, variable.ident.value).as_str());

    if symbol.is_some() {
        let result = compiler.compile_expression(*variable.value);

        return match &result {
            CompilerResult::Exception(_exception) => result,
            _ => CompilerResult::Success,
        };
    } else {
        let var = compiler
            .variable_scope
            .borrow_mut()
            .resolve(format!("{}{}", compiler.location, variable.ident.value));

        if var.is_some() {
            if var.clone().unwrap().modifiers.constant {
                // Program will stop here.
                compiler.throw_exception(String::from("a constant cannot be reassigned"), None);
            }
            compiler.add_to_current_function(format!("{} = ", var.unwrap().transpile()));

            let result = compiler.compile_expression(*variable.value);

            return match &result {
                CompilerResult::Exception(_exception) => result,
                _ => CompilerResult::Success,
            };
        }
    }

    CompilerResult::Exception(CompilerException::UnknownSymbol(UnknownSymbol {
        name: variable.ident.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
