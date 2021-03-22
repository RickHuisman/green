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
            _ => panic!("No opcode for byte: {}", byte),
        }
    }
}