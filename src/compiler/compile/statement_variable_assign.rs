use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::{CompilerException, UnknownSymbol};
use crate::parser::statement::assign::VariableAssign;

pub fn compile_statement_variable_assign(
    compiler: &mut Compiler,
    variable: VariableAssign,
) -> Option<CompilerException> {
    let symbol = compiler
        .symbol_table
        .borrow_mut()
        .resolve(format!("{}{}", compiler.location, variable.ident.value).as_str());

    if symbol.is_some() {
        let error = compiler.compile_expression(*variable.value);

        compiler.emit(OpCode::SetVar, vec![symbol.unwrap().index as u32]);

        return if error.is_some() { error } else { None };
    } else {
        let var = compiler
            .variable_scope
            .borrow_mut()
            .resolve(format!("{}{}", compiler.location, variable.ident.value));

        if var.is_some() {
            let error = compiler.compile_expression(*variable.value);

            compiler.emit(OpCode::SetVar, vec![var.unwrap().index]);

            return if error.is_some() { error } else { None };
        }
    }

    Some(CompilerException::UnknownSymbol(UnknownSymbol {
        name: variable.ident.value,
        scope_depth: compiler.scope_index as u16,
    }))
}
