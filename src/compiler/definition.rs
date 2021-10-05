use crate::compiler::opcode::OpCode;

#[derive(Clone)]
pub struct Definition {
    pub name: String, // pretty printed name instead of a byte representation
    pub operand_width: Vec<i32> // how many bits wide each operand is
}

pub fn get_definition(op: OpCode) -> Definition {
    match op {
        OpCode::Constant => Definition { name: "OpConstant".to_string(), operand_width: vec![2] },
        OpCode::Add => Definition { name: "OpAdd".to_string(), operand_width: vec![] },
        OpCode::Pop => Definition { name: "OpPop".to_string(), operand_width: vec![] },
    }
}