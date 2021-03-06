use crate::compiler::Compiler;
use crate::exception::compiler::{CompilerException, CompilerExceptionCode};
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> Result<Types, CompilerException> {
    let var = compiler.define_symbol(variable.ident.value, variable.data_type.clone(), -1);

    compiler.add_to_current_function(format!(".STORE {} {{", var.index));

    let variable_borrowed = compiler.get_symbol_mutable(var.index, var.name, None);

    let result = compiler.compile_expression(*variable.value);

    let result = if let Ok(_suc_type) = result.clone() {
        compiler.add_to_current_function("};".to_string());

        _suc_type
    } else {
        return result;
    };

    if variable.data_type != result
        && variable.data_type != Types::Auto
        && !matches!(variable.data_type, Types::Module(_))
    {
        return Err(CompilerException::new(
            0,
            0,
            CompilerExceptionCode::WrongType(result.transpile(), variable.data_type.transpile()),
        ));
    }

    // Rc RefCells are so hacky wtf
    if let Some(variable_borrowed) = variable_borrowed {
        if !matches!(
            variable_borrowed.as_ref().borrow_mut()._type,
            Types::Module(_)
        ) {
            variable_borrowed.as_ref().borrow_mut()._type = result;
        }
    }

    Ok(Types::Void)
}
