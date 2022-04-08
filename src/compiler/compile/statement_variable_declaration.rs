use crate::compiler::modifiers::Modifiers;
use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;
use std::ops::DerefMut;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> CompilerResult {
    let var = {
        compiler.variable_scope.borrow_mut().define(
            compiler.variable_count,
            format!("{}{}", compiler.location, variable.ident.value),
            Types::Auto,
            Modifiers::default(),
        )
    };

    compiler.variable_count += 1;
    // let result = compiler.compile_expression(*variable.value);

    // TODO: Make this not auto
    let mut _type = "Variant";
    // This code is for explicit typing, but there need to be checks for the assigned value;
    // let _type = if let Types::Auto = variable.data_type {
    //     "Variant"
    // }
    // else {
    //     variable.data_type.transpile();
    // };

    compiler.add_to_current_function(format!("{} {} = ", _type, var.transpile()));

    let variable_borrowed = compiler
        .variable_scope
        .borrow_mut()
        .get_variable_mutable(var.index, var.name.clone())
        .unwrap()
        .clone();

    let result = compiler.compile_expression(*variable.value, false);

    let result = if let CompilerResult::Success(_type) = result {
        _type
    } else {
        return result;
    };

    // Rc RefCells are so hacky wtf
    variable_borrowed.as_ref().borrow_mut()._type = result;

    CompilerResult::Success(Types::Void)
}
