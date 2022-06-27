use crate::compiler::Compiler;
use crate::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::statement::assign::VariableAssign;
use crate::parser::types::Types;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> Result<Types, CompilerException> {
    let var = compiler.resolve_symbol(&variable.ident.value);

    if let Some(var_type) = var {
        if var_type.modifiers.constant {
            // Program will stop here.
            compiler.throw_exception(String::from("a constant cannot be reassigned"), None);
        }

        compiler.add_to_current_function(format!(".STORE {} {{", var_type.index));

        let result = compiler.compile_expression(*variable.value);

        compiler.add_to_current_function("};".to_string());
        return match &result {
            Err(_exception) => result,
            Ok(result_type) => {
                if *result_type != var_type._type {
                    Err(CompilerException::WrongType(
                        result_type.transpile(),
                        var_type._type.transpile(),
                    ))
                } else {
                    Ok(var_type._type)
                }
            }
        };
    }

    Err(CompilerException::UnknownSymbol(UnknownSymbol {
        name: variable.ident.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
