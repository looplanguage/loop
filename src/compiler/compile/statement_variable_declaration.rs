use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::variable::VariableDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> CompilerResult {
    let var = compiler.variable_scope.borrow_mut().define(
        compiler.variable_count,
        format!("{}{}", compiler.location, variable.ident.value),
        Types::Auto,
    );

    compiler.variable_count += 1;
    // let result = compiler.compile_expression(*variable.value);

    // TODO: Make this not auto
    let mut _type = "Variant";

    compiler.add_to_current_function(format!("{} {} = ", _type, var.transpile()));

    compiler.compile_expression(*variable.value)
}
