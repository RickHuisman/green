use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Chunk {
    name: String,
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            name: "".to_string(),
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, opcode: Opcode, line: usize) {
        self.lines.push(line);
        self.write_byte(opcode as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn code_mut(&mut self) -> &mut Vec<u8> {
        &mut self.code
    }

    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "== {} chunk ==", self.name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = disassemble_instruction(f, self, &mut offset);
        }

        writeln!(f, "")
    }
}

fn disassemble_instruction(f: &mut Formatter<'_>, chunk: &Chunk, offset: &mut usize) -> usize {
    write!(f, "{:04X}", offset);

    // if *offset > 0 &&
    //     chunk.lines[*offset] == chunk.lines[*offset - 1] {
    //     write!(f, "   | ");
    // } else {
    //     write!(f, "{:4} ", chunk.lines[*offset]);
    // }

    write!(f, "   | ");

    let instruction = Opcode::from(chunk.code[*offset]);
    match instruction {
        Opcode::Return => simple_instruction(f, "OP_RETURN", offset),
        Opcode::Constant => constant_instruction(chunk, f, "OP_CONSTANT", offset),
        Opcode::Add => simple_instruction(f, "OP_ADD", offset),
        Opcode::Subtract => simple_instruction(f, "OP_SUBTRACT", offset),
        Opcode::Multiply => simple_instruction(f, "OP_MULTIPLY", offset),
        Opcode::Divide => simple_instruction(f, "OP_DIVIDE", offset),
        Opcode::Print => simple_instruction(f, "OP_PRINT", offset),
        Opcode::Equal => simple_instruction(f, "OP_EQUAL", offset),
        Opcode::Greater => simple_instruction(f, "OP_GREATER", offset),
        Opcode::Less => simple_instruction(f, "OP_LESS", offset),
        Opcode::Not => simple_instruction(f, "OP_NOT", offset),
        Opcode::Negate => simple_instruction(f, "OP_NOT", offset),
        Opcode::DefineGlobal => constant_instruction(chunk, f, "OP_DEFINE_GLOBAL", offset),
        Opcode::GetGlobal => constant_instruction(chunk, f, "OP_GET_GLOBAL", offset),
        Opcode::SetGlobal => constant_instruction(chunk, f, "OP_SET_GLOBAL", offset),
        Opcode::JumpIfFalse => jump_instruction(chunk, f, "OP_JUMP_IF_FALSE", 1, offset),
        Opcode::Jump => jump_instruction(chunk, f, "OP_JUMP", 1, offset),
        Opcode::Pop => simple_instruction(f, "OP_POP", offset),
        Opcode::GetLocal => byte_instruction(chunk, f, "OP_GET_LOCAL", offset),
        Opcode::SetLocal => byte_instruction(chunk, f, "OP_SET_LOCAL", offset),
        Opcode::Nil => simple_instruction(f, "OP_NIL", offset),
        Opcode::Call => byte_instruction(chunk, f, "OP_CALL", offset),
        Opcode::Closure => {
            *offset += 2;

            let constant = chunk.code[*offset - 1];
            write!(f, "{:-16} {:4} ", "OP_CLOSURE", constant);
            writeln!(f, "'{}'", chunk.constants()[constant as usize]);

            *offset
        }
        Opcode::Loop => jump_instruction(chunk, f, "OP_LOOP", 0, offset), // TODO sign should be -1
    }
}

fn simple_instruction(f: &mut Formatter<'_>, name: &str, offset: &mut usize) -> usize {
    writeln!(f, "{}", name);
    *offset + 1
}

fn constant_instruction(
    chunk: &Chunk,
    f: &mut Formatter<'_>,
    name: &str,
    offset: &mut usize,
) -> usize {
    let constant = chunk.code()[*offset + 1];
    write!(f, "{:-16} {:4} ", name, constant);
    writeln!(f, "'{}'", chunk.constants()[constant as usize]);
    *offset + 2
}

fn jump_instruction(
    chunk: &Chunk,
    f: &mut Formatter<'_>,
    name: &str,
    sign: usize,
    offset: &mut usize,
) -> usize {
    let lo = chunk.code[*offset + 2] as u16;
    let hi = chunk.code[*offset + 1] as u16;

    let jump = lo + (hi << 8);

    writeln!(
        f,
        "{:-16} {:4X} -> {:4X}",
        name,
        offset,
        *offset + 3 + sign * jump as usize
    );

    *offset + 3
}

fn byte_instruction(chunk: &Chunk, f: &mut Formatter<'_>, name: &str, offset: &mut usize) -> usize {
    let slot = chunk.code[*offset + 1];
    writeln!(f, "{:-16} {:4X}", name, slot);
    *offset + 2
}
