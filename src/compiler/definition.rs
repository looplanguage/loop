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
        OpCode::Equals => Definition {
            name: "OpEquals".to_string(),
            operand_width: vec![],
        },
        OpCode::NotEquals => Definition {
            name: "OpNotEquals".to_string(),
            operand_width: vec![],
        },
        OpCode::GreaterThan => Definition {
            name: "OpGreaterThan".to_string(),
            operand_width: vec![],
        },
        OpCode::Jump => Definition {
            name: "OpJump".to_string(),
            operand_width: vec![4],
        },
        OpCode::JumpIfFalse => Definition {
            name: "OpJumpIfFalse".to_string(),
            operand_width: vec![4],
        },
        OpCode::Return => Definition {
            name: "OpReturn".to_string(),
            operand_width: vec![],
        },
        OpCode::Function => Definition {
            name: "OpFunction".to_string(),
            operand_width: vec![4, 1],
        },
        OpCode::Call => Definition {
            name: "OpCall".to_string(),
            operand_width: vec![1],
        },
        OpCode::GetLocal => Definition {
            name: "OpGetLocal".to_string(),
            operand_width: vec![1],
        },
        OpCode::GetFree => Definition {
            name: "OpGetFree".to_string(),
            operand_width: vec![1],
        },
        OpCode::GetBuiltin => Definition {
            name: "OpGetBuiltin".to_string(),
            operand_width: vec![1],
        },
        OpCode::CallExtension => Definition {
            name: "OpCallExtension".to_string(),
            operand_width: vec![1, 1],
        },
        OpCode::Array => Definition {
            name: "OpArray".to_string(),
            operand_width: vec![2],
        },
        OpCode::Index => Definition {
            name: "OpIndex".to_string(),
            operand_width: vec![],
        },
        OpCode::AssignIndex => Definition {
            name: "OpAssignIndex".to_string(),
            operand_width: vec![],
        },
        OpCode::Hashmap => Definition {
            name: "OpHashmap".to_string(),
            operand_width: vec![2],
        },
        OpCode::Pow => Definition {
            name: "OpPow".to_string(),
            operand_width: vec![],
        },
        OpCode::Enum => Definition {
            name: "OpEnum".to_string(),
            operand_width: vec![2],
        },
        OpCode::Ident => Definition {
            name: "OpIdent".to_string(),
            operand_width: vec![4]
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
        10 => Some(OpCode::Equals),
        11 => Some(OpCode::NotEquals),
        12 => Some(OpCode::GreaterThan),
        13 => Some(OpCode::Jump),
        14 => Some(OpCode::JumpIfFalse),
        15 => Some(OpCode::Return),
        16 => Some(OpCode::Function),
        17 => Some(OpCode::Call),
        18 => Some(OpCode::GetLocal),
        19 => Some(OpCode::GetFree),
        20 => Some(OpCode::GetBuiltin),
        21 => Some(OpCode::CallExtension),
        22 => Some(OpCode::Array),
        23 => Some(OpCode::Index),
        24 => Some(OpCode::AssignIndex),
        25 => Some(OpCode::Hashmap),
        26 => Some(OpCode::Pow),
        27 => Some(OpCode::Enum),
        28 => Some(OpCode::Ident),
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
        10 => Some(get_definition(OpCode::Equals)),
        11 => Some(get_definition(OpCode::NotEquals)),
        12 => Some(get_definition(OpCode::GreaterThan)),
        13 => Some(get_definition(OpCode::Jump)),
        14 => Some(get_definition(OpCode::JumpIfFalse)),
        15 => Some(get_definition(OpCode::Return)),
        16 => Some(get_definition(OpCode::Function)),
        17 => Some(get_definition(OpCode::Call)),
        18 => Some(get_definition(OpCode::GetLocal)),
        19 => Some(get_definition(OpCode::GetFree)),
        20 => Some(get_definition(OpCode::GetBuiltin)),
        21 => Some(get_definition(OpCode::CallExtension)),
        22 => Some(get_definition(OpCode::Array)),
        23 => Some(get_definition(OpCode::Index)),
        24 => Some(get_definition(OpCode::AssignIndex)),
        25 => Some(get_definition(OpCode::Hashmap)),
        26 => Some(get_definition(OpCode::Pow)),
        27 => Some(get_definition(OpCode::Enum)),
        28 => Some(get_definition(OpCode::Ident)),
        _ => None,
    }
}
