use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use std::borrow::BorrowMut;

pub fn compile_loop_expression(compiler: &mut Compiler, lp: Loop) -> Option<CompilerException> {
    compiler.enter_variable_scope();

    let start = compiler.scope().instructions.len();
    let err = compiler.compile_expression(*lp.condition);

    if err.is_some() {
        return err;
    }

    let done = compiler.emit(OpCode::JumpIfFalse, vec![99999]); // To jump later

    let err = compiler.compile_block(lp.body);

    if err.is_some() {
        return err;
    }

    compiler.emit(OpCode::Jump, vec![start as u32]); // Jump back to start

    compiler.change_operand(
        done as u32,
        vec![compiler.scope().instructions.len() as u32],
    );

    compiler.emit(OpCode::Constant, vec![0]);

    compiler.exit_variable_scope();

    None
}

pub fn compile_loop_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopIterator,
) -> Option<CompilerException> {
    compiler.enter_variable_scope();

    // Define the identifier variable, with the starting integer
    let var = compiler
        .variable_scope
        .as_ref()
        .borrow_mut()
        .define(compiler.variable_count, lp.identifier.value);
    compiler.variable_count += 1;

    // The constant from where we are iterating
    let from = compiler.add_constant(Object::Integer(Integer {
        value: lp.from as i64,
    }));

    // The constant to where we are iterating
    // We do minus 1 as we use the GreaterThan opcode
    let till = compiler.add_constant(Object::Integer(Integer {
        value: lp.till as i64 - 1,
    }));

    // A one constant (to add to the variable)
    let one = compiler.add_constant(Object::Integer(Integer { value: 1 }));

    compiler.emit(OpCode::Constant, vec![from]);

    compiler.emit(OpCode::SetVar, vec![var.index]);

    let start = compiler.scope().instructions.len();

    // Compile the body that is executed
    compiler.compile_block(lp.body);

    // Increase value at the end of body and check if we should go back to start
    compiler.emit(OpCode::Constant, vec![one]);
    compiler.emit(OpCode::GetVar, vec![var.index]);
    compiler.emit(OpCode::Add, vec![]);
    compiler.emit(OpCode::SetVar, vec![var.index]);

    // Check if we should go back to start or not
    compiler.emit(OpCode::GetVar, vec![var.index]); // 10
    compiler.emit(OpCode::Constant, vec![till]); // 9
    compiler.emit(OpCode::GreaterThan, vec![]);
    compiler.emit(OpCode::JumpIfFalse, vec![start as u32]);

    // Emit a null if we didn't break with anything
    compiler.emit(OpCode::Constant, vec![0]);

    compiler.exit_variable_scope();

    None
}

pub fn compile_loop_array_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopArrayIterator,
) -> Option<CompilerException> {
    None
}
