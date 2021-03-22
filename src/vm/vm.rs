use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;
use crate::compiler::opcode::Opcode;

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM { ip: 0, stack: vec![] }
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

    fn print(&mut self) {
        let popped = self.pop(); // TODO should not pop value of stack because it's an expression
        println!("{:?}", popped); // TODO Implement display for Value enum
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        chunk.constants()[self.read_byte(chunk) as usize]
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code()[self.ip];
        self.ip += 1;
        byte
    }

    fn is_at_end(&self, chunk: &Chunk) -> bool {
        self.ip >= chunk.code().len()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Failed to pop value from stack")
    }
}