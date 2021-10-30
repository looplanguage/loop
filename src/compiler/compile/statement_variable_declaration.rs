use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::parser::statement::variable::VariableDeclaration;

pub fn compile_statement_variable_declaration(
    compiler: &mut Compiler,
    variable: VariableDeclaration,
) -> Option<CompilerException> {
    let var = compiler.variable_scope.borrow_mut().define(
        compiler.variable_count,
        format!("{}{}", compiler.location, variable.ident.value),
    );

    compiler.variable_count += 1;
    let err = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![var.index as u32]);

    if err.is_some() {
        return err;
    }

    None
}
