use std::vec;
use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::{CompilerException, UnknownSymbol};
use crate::lib::object::ident::{self, Ident};
use crate::lib::object::Object;
use crate::parser::expression::identifier;

pub fn compile_expression_identifier(compiler: &mut Compiler, identifier: identifier::Identifier) -> Option<CompilerException> {
    let ct = compiler.add_constant(Object::Identifier(ident::Ident {value: identifier.value}));
    compiler.emit(OpCode::Ident, vec![ct]);

    None
}
