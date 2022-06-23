use crate::compiler::{Compiler, CompilerResult};
use crate::exception::compiler::CompilerException;
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> CompilerResult {
    let var = compiler.define_variable(variable.ident.value, variable.data_type.clone(), -1);

    // let result = compiler.compile_expression(*variable.value);

    // TODO: Make this not auto
    let mut _type = "Variant";
    // This code is for explicit typing, but there need to be checks for the assigned value;
    let _type = if let Types::Auto = variable.data_type {
        "Variant".to_string()
    } else {
        variable.data_type.transpile()
    };

    compiler.add_to_current_function(format!(".STORE {} {{", var.index));

    let mut variable_borrowed = None;
    for variable in compiler
        .variable_scope
        .as_ref()
        .borrow_mut()
        .variables
        .clone()
    {
        if variable.as_ref().borrow().name == var.name {
            variable_borrowed = Some(variable);
        }
    }

    let variable_borrowed = variable_borrowed.unwrap();

    let result = compiler.compile_expression(*variable.value);

    let result = if let CompilerResult::Success(_suc_type) = result.clone() {
        compiler.add_to_current_function("};".to_string());

        _suc_type
    } else {
        return result;
    };

    if variable.data_type != result && variable.data_type != Types::Auto {
        return CompilerResult::Exception(CompilerException::WrongType(
            result.transpile(),
            variable.data_type.transpile(),
        ));
    }

    // Rc RefCells are so hacky wtf
    variable_borrowed.as_ref().borrow_mut()._type = result;

    CompilerResult::Success(Types::Void)
}
