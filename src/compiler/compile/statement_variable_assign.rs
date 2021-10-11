use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> Option<String> {
    let symbol = {
        match compiler
            .symbol_table
            .borrow_mut()
            .resolve(variable.ident.value.as_str())
        {
            None => return Some(format!("undefined variable {}", variable.ident.value)),
            Some(symbol) => symbol,
        }
    };

    let error = compiler.compile_expression(*variable.value);

    compiler.emit(OpCode::SetVar, vec![symbol.index as u32]);

    if error.is_some() {
        return error;
    }

    None
}
