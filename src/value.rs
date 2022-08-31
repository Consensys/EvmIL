use crate::{Bytecode,Instruction};

pub enum Value {
    Int(Vec<u8>),
    Hex(Vec<u8>),
}

impl Value {
    pub fn translate(&self, bytecode: &mut Bytecode) {
        match self {
            Value::Int(bytes) => {
                bytecode.push(Instruction::PUSH(bytes.clone()));
            }
            Value::Hex(bytes) => {
                bytecode.push(Instruction::PUSH(bytes.clone()));
            }
        }
    }
}
