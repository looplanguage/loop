use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> CompilerResult {
    let var = compiler
        .variable_scope
        .borrow_mut()
        .resolve(format!("{}{}", compiler.location, variable.ident.value));

    if let Some(var_type) = var {
        if var_type.modifiers.constant {
            // Program will stop here.
            compiler.throw_exception(String::from("a constant cannot be reassigned"), None);
        }

        compiler.add_to_current_function(format!(".STORE {} {{", var_type.index));

        let result = compiler.compile_expression(*variable.value);

        compiler.add_to_current_function("};".to_string());
        return match &result {
            CompilerResult::Exception(_exception) => result,
            CompilerResult::Success(result_type) => {
                if *result_type != var_type._type {
                    CompilerResult::Exception(CompilerException::WrongType(
                        result_type.transpile(),
                        var_type._type.transpile(),
                    ))
                } else {
                    CompilerResult::Success(var_type._type)
                }
            }
            _ => CompilerResult::Exception(CompilerException::Unknown),
        };
    }

    CompilerResult::Exception(CompilerException::UnknownSymbol(UnknownSymbol {
        name: variable.ident.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
