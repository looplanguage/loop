use crate::ast::instructions::suffix::BinaryOperation;
use crate::ast::instructions::Node;
use crate::lexer::token::Instruction;
use crate::parser::error::ParseError;
use crate::parser::instruction::conditional::{
    parse_and_instruction, parse_conditional_instruction, parse_or_instruction,
};
use crate::parser::instruction::function::{
    parse_call_instruction, parse_function_instruction, parse_return_instruction,
};
use crate::parser::instruction::memory::{parse_assign_instruction, parse_compound_instruction, parse_constant_instruction, parse_copy_instruction, parse_index_instruction, parse_length_instruction, parse_load_instruction, parse_loadlib_instruction, parse_pop_instruction, parse_push_instruction, parse_slice_instruction, parse_store_instruction};
use crate::parser::instruction::suffix::parse_math_instruction;
use crate::parser::instruction::while_loop::parse_while_instruction;
use crate::parser::Parser;

mod conditional;
mod function;
mod memory;
mod suffix;
mod while_loop;

pub fn parse_instruction(parser: &mut Parser, ins: Instruction) -> Result<Node, ParseError> {
    match ins {
        Instruction::CONSTANT => parse_constant_instruction(parser),
        Instruction::LOAD => parse_load_instruction(parser),
        Instruction::STORE => parse_store_instruction(parser),
        Instruction::ADD => parse_math_instruction(parser, BinaryOperation::ADD),
        Instruction::SUBTRACT => parse_math_instruction(parser, BinaryOperation::SUBTRACT),
        Instruction::MULTIPLY => parse_math_instruction(parser, BinaryOperation::MULTIPLY),
        Instruction::DIVIDE => parse_math_instruction(parser, BinaryOperation::DIVIDE),
        Instruction::POWER => parse_math_instruction(parser, BinaryOperation::POWER),
        Instruction::GREATERTHAN => parse_math_instruction(parser, BinaryOperation::GREATERTHAN),
        Instruction::EQUALS => parse_math_instruction(parser, BinaryOperation::EQUALS),
        Instruction::NOTEQUALS => parse_math_instruction(parser, BinaryOperation::NOTEQUALS),
        Instruction::IF => parse_conditional_instruction(parser),
        Instruction::FUNCTION => parse_function_instruction(parser),
        Instruction::CALL => parse_call_instruction(parser),
        Instruction::WHILE => parse_while_instruction(parser),
        Instruction::INDEX => parse_index_instruction(parser),
        Instruction::PUSH => parse_push_instruction(parser),
        Instruction::SLICE => parse_slice_instruction(parser),
        Instruction::COPY => parse_copy_instruction(parser),
        Instruction::LOADLIB => parse_loadlib_instruction(parser),
        Instruction::RETURN => parse_return_instruction(parser),
        Instruction::ASSIGN => parse_assign_instruction(parser),
        Instruction::POP => parse_pop_instruction(parser),
        Instruction::LENGTH => parse_length_instruction(parser),
        Instruction::AND => parse_and_instruction(parser),
        Instruction::OR => parse_or_instruction(parser),
        Instruction::MODULO => parse_math_instruction(parser, BinaryOperation::MODULO),
        Instruction::COMPOUND => parse_compound_instruction(parser),
    }
}
