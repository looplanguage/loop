use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::constant::ConstantDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_constant_declaration(
    compiler: &mut Compiler,
    constant: ConstantDeclaration,
) -> CompilerResult {
    let var = compiler.define_variable(
        format!("{}{}", compiler.location, constant.ident.value),
        constant.data_type.clone(),
    );
    // let result = compiler.compile_expression(*variable.value);

    // ToDo: Check whether value has the same type as the type, otherwise there will be a D error
    let _type = constant.data_type.transpile();

    compiler.add_to_current_function(format!("const {} {} = ", _type, var.transpile()));

    let result = compiler.compile_expression(*constant.value, false);

    if let CompilerResult::Success(inferred_type) = result {
        compiler.replace_at_current_function(
            format!("const {} {} = ", _type, var.transpile()),
            format!("const {} {} = ", inferred_type.transpile(), var.transpile()),
        );
    }

    // compiler.emit(OpCode::SetVar, vec![var.index as u32]);

    CompilerResult::Success(Types::Void)
}
