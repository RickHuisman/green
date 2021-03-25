use crate::compiler::chunk::Chunk;
use crate::compiler::value::{Value, value_to_string};
use crate::compiler::opcode::Opcode;
use std::collections::HashMap;
use crate::compiler::object::Object;

pub struct VM {
    ip: usize,
    stack_top: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM { ip: 0, stack_top: 0, stack: vec![], globals: HashMap::new() }
    }

    pub fn interpret(&mut self, chunk: &Chunk) {
        self.run(chunk)
    }

    fn run(&mut self, chunk: &Chunk) {
        while !self.is_at_end(chunk) {
            let instruction = Opcode::from(self.read_byte(chunk));
            match instruction {
                Opcode::Return => self.ret(),
                Opcode::Constant => self.constant(&chunk),
                Opcode::Add => self.add(),
                Opcode::Subtract => self.subtract(),
                Opcode::Multiply => self.multiply(),
                Opcode::Divide => self.divide(),
                Opcode::Print => self.print(),
                Opcode::Equal => self.equal(),
                Opcode::Greater => self.greater(),
                Opcode::Less => self.less(),
                Opcode::Not => self.not(),
                Opcode::Negate => self.negate(),
                Opcode::DefineGlobal => self.define_global(chunk),
                Opcode::GetGlobal => self.get_global(chunk),
                Opcode::SetGlobal => self.set_global(chunk),
                Opcode::JumpIfFalse => self.jump_if_false(chunk),
                Opcode::Jump => self.jump(chunk),
                Opcode::Pop => { self.pop(); },
                Opcode::GetLocal => self.get_local(chunk),
                Opcode::SetLocal => self.set_local(chunk),
            }
        }
    }

    fn ret(&mut self) {
        let popped = self.pop();
        println!("{:?}", popped); // TODO
    }

    fn constant(&mut self, chunk: &Chunk) {
        let constant = self.read_constant(chunk);
        self.push(constant);
    }

    fn add(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a + b);
    }

    fn subtract(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a - b);
    }

    fn multiply(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a * b);
    }

    fn divide(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(a / b);
    }

    fn equal(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push((a == b).into());
    }

    fn greater(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push((a > b).into());
    }

    fn less(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push((a < b).into());
    }

    fn not(&mut self) {
        let a = self.pop();
        self.push(bool::into(bool::from(a)));
    }

    fn negate(&mut self) {
        let a = self.pop();
        self.push(-a);
    }

    fn define_global(&mut self, chunk: &Chunk) {
        let name = value_to_string(self.read_constant(chunk));
        let val = self.peek(0);
        self.globals.insert(name, val);
        self.pop();
    }

    fn get_global(&mut self, chunk: &Chunk) {
        let name = value_to_string(self.read_constant(chunk));
        let value = self.globals.get(&name).cloned();
        self.push(value.unwrap()); // TODO Unwrap???
    }

    fn set_global(&mut self, chunk: &Chunk) {
        let name = value_to_string(self.read_constant(chunk));
        let peek = self.peek(0);
        if let Some(global) = self.globals.get_mut(&name) {
            *global = peek;
        } else {
            panic!("No global with name: {}", name);
        }
    }

    fn jump_if_false(&mut self, chunk: &Chunk) {
        let offset = self.read_short(chunk);

        if bool::from(self.peek(0)) {
            self.ip += offset as usize;
        }
    }

    fn jump(&mut self, chunk: &Chunk) {
        let offset = self.read_short(chunk);
        self.ip += offset as usize;
    }

    fn print(&mut self) {
        let popped = self.pop(); // TODO should not pop value of stack because it's an expression
        println!("{:?}", popped); // TODO Implement display for Value enum
    }

    fn get_local(&mut self, chunk: &Chunk) {
        let slot = self.read_byte(chunk);
        self.push(self.stack[slot as usize].clone()); // TODO Clone???
    }

    fn set_local(&mut self, chunk: &Chunk) {
        let slot = self.read_byte(chunk);
        let peek = self.peek(0);
        if let Some(local) = self.stack.get_mut(slot as usize) {
            *local = peek;
        } else {
            panic!("No local with name: {}", "TODO"); // TODO Name of local
        }
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let constant_index = self.read_byte(chunk);
        chunk.constants()[constant_index as usize].clone()
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code()[self.ip];
        self.ip = self.ip + 1;
        byte
    }

    fn read_short(&mut self, chunk: &Chunk) -> u16 {
        self.ip += 2;

        let lo = chunk.code()[self.ip - 2] as u16;
        let hi = chunk.code()[self.ip - 1] as u16;
        (lo << 8) | hi
    }

    fn is_at_end(&self, chunk: &Chunk) -> bool {
        self.ip >= chunk.code().len()
    }

    fn push(&mut self, value: Value) {
        self.stack_top += 1;
        self.stack.push(value);
    }

    fn peek(&mut self, offset: usize) -> Value {
        self.stack[self.stack_top - 1 - offset].clone()
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack.pop().expect("Failed to pop value from stack")
    }
}