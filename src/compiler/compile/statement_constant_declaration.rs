use crate::compiler::modifiers::Modifiers;
use crate::compiler::{Compiler, CompilerResult};
use crate::parser::statement::constant::ConstantDeclaration;
use crate::parser::types::Types;

pub fn compile_statement_constant_declaration(
    compiler: &mut Compiler,
    constant: ConstantDeclaration,
) -> CompilerResult {
    let var = compiler.variable_scope.borrow_mut().define(
        compiler.variable_count,
        format!("{}{}", compiler.location, constant.ident.value),
        constant.data_type.clone(),
        Modifiers::new(true),
    );

    compiler.variable_count += 1;
    // let result = compiler.compile_expression(*variable.value);

    // ToDo: Check whether value has the same type as the type, otherwise there will be a D error
    let _type = constant.data_type.transpile();

    compiler.add_to_current_function(format!("const {} {} = ", _type, var.transpile()));

    compiler.compile_expression(*constant.value, false);

    // compiler.emit(OpCode::SetVar, vec![var.index as u32]);

    CompilerResult::Success(Types::Void)
}
