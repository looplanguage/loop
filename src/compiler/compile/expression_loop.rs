use crate::compiler::opcode::OpCode;
use crate::compiler::Compiler;
use crate::lib::exception::compiler::CompilerException;
use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use crate::parser::expression::{array, integer, Expression};

pub fn compile_loop_expression(compiler: &mut Compiler, lp: Loop) -> Option<CompilerException> {
    compiler.enter_variable_scope();

    let start = compiler.scope().instructions.len();
    let err = compiler.compile_expression(*lp.condition);

    compiler.emit(OpCode::StartSection, vec![]);

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

    // Set all breaks to go to the end of current loop
    for br in compiler.breaks.clone() {
        compiler.change_operand(br, vec![compiler.scope().instructions.len() as u32])
    }

    compiler.breaks = vec![];

    compiler.emit(OpCode::Constant, vec![0]);

    compiler.exit_variable_scope();

    compiler.emit(OpCode::EndSection, vec![]);

    None
}

pub fn compile_loop_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopIterator,
) -> Option<CompilerException> {
    compiler.enter_variable_scope();
    compiler.emit(OpCode::StartSection, vec![]);

    // Define the identifier variable, with the starting integer
    let var = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        lp.identifier.value,
        Expression::Integer(integer::Integer { value: 0 }),
    );
    compiler.variable_count += 1;

    // The constant from where we are iterating
    let from = compiler.add_constant(Object::Integer(Integer {
        value: lp.from as i64,
    }));

    // The constant to where we are iterating
    // We do minus 1 as we use the GreaterThan opcode
    let till = compiler.add_constant(Object::Integer(Integer {
        value: lp.till as i64,
    }));

    // A one constant (to add to the variable)
    let one = compiler.add_constant(Object::Integer(Integer { value: 1 }));

    // Set the initial value

    compiler.emit(OpCode::Constant, vec![from]);
    compiler.emit(OpCode::SetVar, vec![var.index]);

    // Check if we should skip over
    let start = compiler.scope().instructions.len();
    compiler.emit(OpCode::Constant, vec![till]); // 2
    compiler.emit(OpCode::GetVar, vec![var.index]); // 1
    compiler.emit(OpCode::GreaterThan, vec![]); // 1
    let start_jump = compiler.emit(OpCode::JumpIfFalse, vec![start as u32]); // 0

    // Compile the body that is executed
    compiler.compile_loop_block(lp.body);

    // Increase value at the end of body and check if we should go back to start
    compiler.emit(OpCode::Constant, vec![one]);
    compiler.emit(OpCode::GetVar, vec![var.index]);
    compiler.emit(OpCode::Add, vec![]);
    compiler.emit(OpCode::SetVar, vec![var.index]); // 0

    // Emit a null if we didn't break with anything
    compiler.emit(OpCode::Constant, vec![0]);

    // Jump to start
    compiler.emit(OpCode::Jump, vec![start as u32]);

    // Change JumpIfFalse to the end
    compiler.emit(OpCode::Constant, vec![0]);
    compiler.change_operand(
        start_jump as u32,
        vec![compiler.scope().instructions.len() as u32 + 1],
    );

    // Set all breaks to go to the end of current loop
    for br in compiler.breaks.clone() {
        compiler.change_operand(br, vec![compiler.scope().instructions.len() as u32])
    }

    compiler.breaks = vec![];

    compiler.exit_variable_scope();
    compiler.emit(OpCode::EndSection, vec![]);

    None
}

pub fn compile_loop_array_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopArrayIterator,
) -> Option<CompilerException> {
    compiler.enter_variable_scope();
    compiler.emit(OpCode::StartSection, vec![]);

    // Put the array on the stack and assign it to a cache variable
    let array = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        "/iterator-array".to_string(),
        Expression::Array(Box::from(array::Array { values: vec![] })),
    );
    compiler.variable_count += 1;

    compiler.compile_expression(*lp.array);
    compiler.emit(OpCode::SetVar, vec![array.index]);

    // Define the identifier variable, with the starting value of the array
    let var = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        lp.identifier.value,
        Expression::Integer(integer::Integer { value: 0 }),
    );
    compiler.variable_count += 1;

    let index = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        "/iterator-index".to_string(),
        Expression::Integer(integer::Integer { value: 0 }),
    );
    compiler.variable_count += 1;

    // Set index to 0
    let zero = compiler.add_constant(Object::Integer(Integer { value: 0 }));
    let one = compiler.add_constant(Object::Integer(Integer { value: 1 }));

    compiler.emit(OpCode::Constant, vec![zero]);
    compiler.emit(OpCode::SetVar, vec![index.index]);

    // Check length before we assign and continue to the next one
    // Skip to end if length is lower than our index
    let start = compiler.scope().instructions.len();
    compiler.emit(OpCode::GetBuiltin, vec![0]); // 0 = len builtin
    compiler.emit(OpCode::GetVar, vec![array.index]);
    compiler.emit(OpCode::Call, vec![1]); // actually call to check length
    compiler.emit(OpCode::GetVar, vec![index.index]); // Get the index of the current iteration
    compiler.emit(OpCode::GreaterThan, vec![]); // Get the index of the current iteration
    let end = compiler.emit(OpCode::JumpIfFalse, vec![999999]); // Create a jump to end if it is greater, otherwise increase index

    compiler.emit(OpCode::GetVar, vec![array.index]); // Get the array
    compiler.emit(OpCode::GetVar, vec![index.index]); // Get the index
    compiler.emit(OpCode::Index, vec![]); // Index the array
    compiler.emit(OpCode::SetVar, vec![var.index]); // Assign it to the variable

    // Compile body and then increase the index
    compiler.compile_block(lp.body);

    compiler.emit(OpCode::Constant, vec![one]);
    compiler.emit(OpCode::GetVar, vec![index.index]); // Get the index
    compiler.emit(OpCode::Add, vec![]); // Add to the index
    compiler.emit(OpCode::SetVar, vec![index.index]); // Assign the new value to it
    compiler.emit(OpCode::Jump, vec![start as u32]); // And go back to start

    // set jump to end & add a final "null" value for if we didn't break inside the loop
    compiler.change_operand(end as u32, vec![compiler.scope().instructions.len() as u32]);

    // Set all breaks to go to the end of current loop
    for br in compiler.breaks.clone() {
        compiler.change_operand(br, vec![compiler.scope().instructions.len() as u32])
    }

    compiler.breaks = vec![];

    compiler.emit(OpCode::Constant, vec![0]);
    compiler.emit(OpCode::EndSection, vec![]);

    None
}
