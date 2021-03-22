use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;
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
        self.push((a > b).into());
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
        // FIXME
        let name = self.read_constant(chunk);
        match name {
            Value::Obj(s) => {
                match s {
                    Object::String(s) => {
                        let val = self.peek(0);
                        self.globals.insert(s, val);
                        self.pop();
                    }
                }
            }
            _ => panic!("TODO")
        }
    }

    fn get_global(&mut self, chunk: &Chunk) {
        // FIXME
        let name = self.read_constant(chunk);
        match name {
            Value::Obj(s) => {
                match s {
                    Object::String(s) => {
                        let value = self.globals.get(&s).cloned();
                        self.push(value.unwrap());
                    }
                }
            }
            _ => panic!("TODO")
        }
    }

    fn print(&mut self) {
        let popped = self.pop(); // TODO should not pop value of stack because it's an expression
        println!("{:?}", popped); // TODO Implement display for Value enum
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