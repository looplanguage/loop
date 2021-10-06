use crate::compiler::definition::{get_definition, lookup, Definition};
use crate::compiler::opcode::OpCode;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub type Instructions = Vec<u8>;

pub fn pretty_print_instructions(ins: Instructions) -> String {
    let mut i: i32 = 0;
    let mut inst: Vec<String> = vec![];

    while i < ins.len() as i32 {
        let def = lookup(ins[i as usize]);
        if def.is_none() {
            return format!("operand not found {}", ins[i as usize]);
        }

        let definition = def.clone().unwrap();

        let data = read_operands(definition.clone(), ins[((i + 1_i32) as usize)..].to_owned());
        let _operands = data.0;
        let read = data.1;

        let mut operand_data: Vec<String> = vec![];

        for _operand in _operands {
            operand_data.push(_operand.to_string());
        }

        if operand_data.is_empty() {
            inst.push(format!("[{}] {}", i, definition.name));
        } else {
            inst.push(format!(
                "[{}] {} {}",
                i,
                definition.name,
                operand_data.join(" ")
            ));
        }

        i += 1 + read;

        if i < ins.len() as i32 {
            inst.push("\n".to_string())
        }
    }

    inst.concat()
}

pub fn print_instructions(ins: Instructions) {
    println!("{}", pretty_print_instructions(ins))
}

pub fn read_operands(def: Definition, ins: Vec<u8>) -> (Vec<i64>, i32) {
    let mut operands: Vec<i64> = vec![0; def.operand_width.len()];
    let mut offset = 0;

    for (i, width) in def.operand_width.iter().enumerate() {
        match *width {
            4 => operands[i] = read_uint32(ins[offset..].to_owned()) as i64,
            2 => operands[i] = read_uint16(ins[offset..].to_owned()) as i64,
            1 => operands[i] = read_uint8(ins[offset..].to_owned()) as i64,
            _ => {}
        }

        offset += (*width) as usize
    }

    (operands, offset as i32)
}

pub fn read_uint8(ins: Vec<u8>) -> u8 {
    let mut rdr = Cursor::new(ins);
    let try_read = rdr.read_u8();

    if try_read.is_err() {
        return u8::MAX;
    }

    try_read.unwrap()
}

pub fn read_uint16(ins: Instructions) -> u16 {
    let mut rdr = Cursor::new(ins);
    let try_read = rdr.read_u16::<BigEndian>();
    if try_read.is_err() {
        return u16::MAX;
    }

    try_read.unwrap()
}

pub fn read_uint32(ins: Instructions) -> u32 {
    let mut rdr = Cursor::new(ins);
    let try_read = rdr.read_u32::<BigEndian>();
    if try_read.is_err() {
        return u32::MAX;
    }

    try_read.unwrap()
}

pub fn make_instruction(op: OpCode, operands: Vec<u32>) -> Vec<u8> {
    let mut ins_length = 1;

    let def = get_definition(op);

    for width in &def.operand_width {
        ins_length += width;
    }

    let mut instruction: Vec<u8> = vec![op as u8];

    for (key, val) in operands.iter().enumerate() {
        let width = def.operand_width[key];

        let result = match width {
            4 => instruction.write_u32::<BigEndian>(* val as u32),
            2 => instruction.write_u16::<BigEndian>(*val as u16),
            _ => instruction.write_u8(*val as u8),
        };

        if result.is_err() {
            // TODO: Add compiler error
        }
    }

    instruction
}
