use crate::compiler::value::Value;
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::compiler::Opcode::Opcode;

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: vec![], constants: vec![], lines: vec![] }
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

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn constants(&self) -> &Vec<Value> {
        &self.constants
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "== TODO chunk ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = disassemble_instruction(f, self, &mut offset);
        }

        writeln!(f, "")
    }
}

fn disassemble_instruction(f: &mut Formatter<'_>, chunk: &Chunk, offset: &mut usize) -> usize {
    write!(f, "{:04}", offset);

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
        Opcode::Constant => constant_instruction(
            chunk,
            f,
            "OP_CONSTANT",
            offset,
        ),
        Opcode::Add => simple_instruction(f, "OP_ADD", offset),
        Opcode::Subtract => simple_instruction(f, "OP_SUBTRACT", offset),
        Opcode::Multiply => simple_instruction(f, "OP_MULTIPLY", offset),
        Opcode::Divide => simple_instruction(f, "OP_DIVIDE", offset),
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