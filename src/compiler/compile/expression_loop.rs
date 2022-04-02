use crate::compiler::opcode::OpCode;
use crate::compiler::{Compiler, CompilerResult};
use crate::lib::object::integer::Integer;
use crate::lib::object::Object;
use crate::parser::expression::loops::{Loop, LoopArrayIterator, LoopIterator};
use crate::parser::expression::{array, integer, Expression};

pub fn compile_loop_expression(compiler: &mut Compiler, lp: Loop) -> CompilerResult {
    compiler.enter_variable_scope();

    // Condition
    compiler.add_to_current_function("while (".to_string());
    let result = compiler.compile_expression(*lp.condition);
    compiler.add_to_current_function(") ".to_string());

    // Body
    let result = compiler.compile_block(lp.body);

    CompilerResult::Success
}

pub fn compile_loop_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopIterator,
) -> CompilerResult {
    compiler.enter_variable_scope();
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

    compiler.add_to_current_function(format!("int {} = {};", var.transpile(), lp.from));

    // The constant to where we are iterating
    // We do minus 1 as we use the GreaterThan opcode
    let till = compiler.add_constant(Object::Integer(Integer {
        value: lp.till as i64,
    }));

    compiler.add_to_current_function(format!("while({} < {}) {{", var.transpile(), lp.till));

    // Compile the body that is executed
    compiler.compile_loop_block(lp.body);

    compiler.exit_variable_scope();

    // Increase it
    compiler.add_to_current_function(format!("{} += 1; }}", var.transpile()));

    CompilerResult::Success
}

pub fn compile_loop_array_iterator_expression(
    compiler: &mut Compiler,
    lp: LoopArrayIterator,
) -> CompilerResult {
    compiler.enter_variable_scope();

    // Put the array on the stack and assign it to a cache variable
    let array = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        "_iterator_array".to_string(),
        Expression::Array(Box::from(array::Array { values: vec![] })),
    );
    compiler.variable_count += 1;

    // Array
    compiler.add_to_current_function(format!("auto {} = ", array.transpile()));
    compiler.compile_expression(*lp.array);
    compiler.add_to_current_function(";".to_string());

    // Define the identifier variable, with the starting value of the array
    let var = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        lp.identifier.value,
        Expression::Integer(integer::Integer { value: 0 }),
    );
    compiler.variable_count += 1;

    let index = compiler.variable_scope.as_ref().borrow_mut().define(
        compiler.variable_count,
        "_iterator_index".to_string(),
        Expression::Integer(integer::Integer { value: 0 }),
    );
    compiler.variable_count += 1;

    compiler.add_to_current_function(format!("int {} = 0;", index.transpile()));
    compiler.add_to_current_function(format!("auto {} = {}[0];", var.transpile(), array.transpile()));

    compiler.add_to_current_function(format!("while({} < {}.length) {{ ", index.transpile(), array.transpile()));

    // Compile body and then increase the index
    compiler.compile_loop_block(lp.body);

    compiler.add_to_current_function(format!("{} += 1;", index.transpile()));
    compiler.add_to_current_function(format!("if({} < {}.length) {{ {} = {}[{}]; }} }}", index.transpile(), array.transpile(), var.transpile(), array.transpile(), index.transpile()));

    CompilerResult::Success
}
