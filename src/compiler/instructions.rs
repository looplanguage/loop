use crate::compiler::definition::{get_definition, lookup, Definition};
use crate::compiler::opcode::OpCode;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub type Instructions = Vec<u8>;

pub fn print_instructions(ins: Instructions) {
    let mut i: i32 = 0;

    while i < ins.len() as i32 {
        let def = lookup(ins[i as usize]);
        if def.is_none() {
            return;
        }

        let definition = def.clone().unwrap();

        let data = read_operands(
            definition.clone(),
            ins[((i + 1 as i32) as usize)..].to_owned(),
        );
        let _operands = data.0;
        let read = data.1;

        let mut operand_data: Vec<String> = vec![];

        for _operand in _operands {
            operand_data.push(_operand.to_string() + " ");
        }

        println!("[{}] {} {}", i, definition.name, operand_data.concat());

        i += 1 + read
    }
}

pub fn read_operands(def: Definition, ins: Vec<u8>) -> (Vec<i32>, i32) {
    let mut operands: Vec<i32> = vec![0; def.operand_width.len()];
    let mut offset = 0;

    for (i, width) in def.operand_width.iter().enumerate() {
        match width {
            &2 => operands[i] = read_uint16(ins[offset..].to_owned()) as i32,
            &1 => operands[i] = read_uint8(ins[offset..].to_owned()) as i32,
            &_ => {}
        }

        offset += (*width) as usize
    }

    (operands, offset as i32)
}

fn read_uint8(ins: Vec<u8>) -> u8 {
    let mut rdr = Cursor::new(ins);
    let try_read = rdr.read_u8();

    if try_read.is_err() {
        return u8::MAX;
    }

    try_read.unwrap()
}

fn read_uint16(ins: Instructions) -> u16 {
    let mut rdr = Cursor::new(ins);
    let try_read = rdr.read_u16::<BigEndian>();
    if try_read.is_err() {
        return u16::MAX;
    }

    try_read.unwrap()
}

pub fn make_instruction(op: OpCode, operands: Vec<u16>) -> Vec<u8> {
    let mut ins_length = 1;

    let def = get_definition(op);

    for width in &def.operand_width {
        ins_length += width;
    }

    let mut instruction: Vec<u8> = vec![];

    instruction.push(op as u8);

    let mut offset = 1;
    for (key, val) in operands.iter().enumerate() {
        let width = def.operand_width[key];

        match width {
            2 => {
                instruction.write_u16::<BigEndian>(*val as u16);
            }
            _ => {
                instruction.write_u8(*val as u8);
            }
        };

        offset += width as usize;
    }

    return instruction;
}
