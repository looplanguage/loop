use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::constant::ConstantDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_constant_declaration(
    compiler: &mut Compiler,
    constant: ConstantDeclaration,
) -> CompilerResult {
    let var = compiler.define_variable(constant.ident.value, constant.data_type.clone(), -1);
    // let result = compiler.compile_expression(*variable.value);

    // TODO: Check whether value has the same type as the type, otherwise there will be a D error
    let _type = constant.data_type.transpile();

    compiler.add_to_current_function(format!(".STORE {} {{", var.index));

    let _ = compiler.compile_expression(*constant.value);

    compiler.add_to_current_function("};".to_string());

    // compiler.emit(OpCode::SetVar, vec![var.index as u32]);

    CompilerResult::Success(Types::Void)
}
