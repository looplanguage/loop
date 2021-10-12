use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> Option<String> {
    let symbol = compiler
        .symbol_table
        .borrow_mut()
        .resolve(variable.ident.value.as_str());

    if symbol.is_some() {
        let error = compiler.compile_expression(*variable.value);

        compiler.emit(OpCode::SetVar, vec![symbol.unwrap().index as u32]);

        return if error.is_some() {
            error
        } else {
            None
        }
    } else {
        let var = compiler.variable_scope.borrow_mut().resolve(variable.ident.value.clone());

        if var.is_some() {
            let error = compiler.compile_expression(*variable.value);

            compiler.emit(OpCode::SetVar, vec![var.unwrap().index]);

            return if error.is_some() {
                error
            } else {
                None
            }
        }
    }

    Some(format!("undefined variable {}", variable.ident.value))
}
