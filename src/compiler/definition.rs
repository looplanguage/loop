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
            operand_width: vec![4],
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
        OpCode::Modulo => Definition {
            name: "OpModulo".to_string(),
            operand_width: vec![],
        },
        OpCode::Minus => Definition {
            name: "OpMinus".to_string(),
            operand_width: vec![],
        },
        OpCode::Multiply => Definition {
            name: "OpMultiply".to_string(),
            operand_width: vec![],
        },
        OpCode::Divide => Definition {
            name: "OpDivide".to_string(),
            operand_width: vec![],
        },
        OpCode::SetVar => Definition {
            name: "OpSetVar".to_string(),
            operand_width: vec![4],
        },
        OpCode::GetVar => Definition {
            name: "OpGetVar".to_string(),
            operand_width: vec![4],
        },
    }
}

pub fn lookup_op(op: u8) -> Option<OpCode> {
    match op {
        0 => Some(OpCode::Constant),
        1 => Some(OpCode::Add),
        2 => Some(OpCode::Pop),
        3 => Some(OpCode::Closure),
        4 => Some(OpCode::Modulo),
        5 => Some(OpCode::Multiply),
        6 => Some(OpCode::Divide),
        7 => Some(OpCode::Minus),
        8 => Some(OpCode::SetVar),
        9 => Some(OpCode::GetVar),
        _ => None,
    }
}

pub fn lookup(op: u8) -> Option<Definition> {
    match op {
        0 => Some(get_definition(OpCode::Constant)),
        1 => Some(get_definition(OpCode::Add)),
        2 => Some(get_definition(OpCode::Pop)),
        3 => Some(get_definition(OpCode::Closure)),
        4 => Some(get_definition(OpCode::Modulo)),
        5 => Some(get_definition(OpCode::Multiply)),
        6 => Some(get_definition(OpCode::Divide)),
        7 => Some(get_definition(OpCode::Minus)),
        8 => Some(get_definition(OpCode::SetVar)),
        9 => Some(get_definition(OpCode::GetVar)),
        _ => None,
    }
}
