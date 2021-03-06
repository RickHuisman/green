use crate::syntax::expr::UnaryOperator;

#[repr(u8)]
pub enum Opcode {
    Return,
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Print,
    Equal,
    Greater,
    Less,
    Not,
    Negate,
    DefineGlobal,
    GetGlobal,
    SetGlobal,
    JumpIfFalse,
    Jump,
    Pop,
    GetLocal,
    SetLocal,
    Nil,
    Call,
    Closure,
    Loop,

    // Array opcodes
    NewArray,
    IndexSubscript,
    StoreSubscript,

    Class,
    GetProperty,
    SetProperty,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Opcode::Return,
            0x01 => Opcode::Constant,
            0x02 => Opcode::Add,
            0x03 => Opcode::Subtract,
            0x04 => Opcode::Multiply,
            0x05 => Opcode::Divide,
            0x06 => Opcode::Print,
            0x07 => Opcode::Equal,
            0x08 => Opcode::Greater,
            0x09 => Opcode::Less,
            0x0a => Opcode::Not,
            0x0b => Opcode::Negate,
            0x0c => Opcode::DefineGlobal,
            0x0d => Opcode::GetGlobal,
            0x0e => Opcode::SetGlobal,
            0x0f => Opcode::JumpIfFalse,
            0x10 => Opcode::Jump,
            0x11 => Opcode::Pop,
            0x12 => Opcode::GetLocal,
            0x13 => Opcode::SetLocal,
            0x14 => Opcode::Nil,
            0x15 => Opcode::Call,
            0x16 => Opcode::Closure,
            0x17 => Opcode::Loop,
            0x18 => Opcode::NewArray,
            0x19 => Opcode::IndexSubscript,
            26 => Opcode::StoreSubscript, // TODO
            27 => Opcode::Class,          // TODO
            28 => Opcode::GetProperty,    // TODO
            29 => Opcode::SetProperty,    // TODO
            _ => panic!("No opcode for byte: {}", byte),
        }
    }
}

impl From<UnaryOperator> for Opcode {
    fn from(op: UnaryOperator) -> Self {
        match op {
            UnaryOperator::Negate => Opcode::Negate,
            UnaryOperator::Not => Opcode::Not,
        }
    }
}
