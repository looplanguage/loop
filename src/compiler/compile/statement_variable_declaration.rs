use crate::compiler::modifiers::Modifiers;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> CompilerResult {
    let var = {
        compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!("{}{}", compiler.location, variable.ident.value),
            variable.data_type.clone(),
            Modifiers::default(),
        )
    };

    compiler.variable_count += 1;
    // let result = compiler.compile_expression(*variable.value);

    // TODO: Make this not auto
    let mut _type = "Variant";
    // This code is for explicit typing, but there need to be checks for the assigned value;
    let _type = if let Types::Auto = variable.data_type {
        "Variant".to_string()
    } else {
        variable.data_type.transpile()
    };

    compiler.add_to_current_function(format!("{} {} = ", _type, var.transpile()));

    let variable_borrowed = compiler
        .variable_scope
        .borrow_mut()
        .get_variable_mutable(var.index, var.name.clone())
        .unwrap();

    let result = compiler.compile_expression(*variable.value, false);

    let result = if let CompilerResult::Success(_suc_type) = result.clone() {
        if variable.data_type != Types::Auto && _suc_type != variable.data_type {
            return CompilerResult::Exception(CompilerException::WrongType(
                _suc_type.transpile(),
                variable.data_type.transpile(),
            ));
        }

        if variable.data_type == Types::Auto {
            if let CompilerResult::Success(inferred_type) = result {
                compiler.replace_at_current_function(
                    format!("{} {} = ", _type, var.transpile()),
                    format!("{} {} = ", inferred_type.transpile(), var.transpile()),
                );
            }
        }

        _suc_type
    } else {
        return result;
    };

    // Rc RefCells are so hacky wtf
    variable_borrowed.as_ref().borrow_mut()._type = result;

    CompilerResult::Success(Types::Void)
}
