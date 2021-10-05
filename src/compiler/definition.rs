use crate::compiler::opcode::OpCode;

#[derive(Clone)]
pub struct Definition {
    pub name: String,            // pretty printed name instead of a byte representation
    pub operand_width: Vec<i32>, // how many bits wide each operand is
}

pub fn get_definition(op: OpCode) -> Definition {
    match op {
        OpCode::Constant => Definition {
            name: "OpConstant".to_string(),
            operand_width: vec![2],
        },
        OpCode::Add => Definition {
            name: "OpAdd".to_string(),
            operand_width: vec![],
        },
        OpCode::Pop => Definition {
            name: "OpPop".to_string(),
            operand_width: vec![],
        },
        OpCode::Closure => Definition {
            name: "OpClosure".to_string(),
            operand_width: vec![2, 1],
        },
    }
}

pub fn lookup(op: u8) -> Option<Definition> {
    match op {
        0 => Some(get_definition(OpCode::Constant)),
        1 => Some(get_definition(OpCode::Add)),
        2 => Some(get_definition(OpCode::Pop)),
        3 => Some(get_definition(OpCode::Closure)),
        _ => None,
    }
}
