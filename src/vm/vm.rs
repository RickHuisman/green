use crate::compiler::chunk::Chunk;
use crate::compiler::value::{Value, value_to_string};
use crate::compiler::opcode::Opcode;
use std::collections::HashMap;
use crate::compiler::object::Object;
use crate::vm::callframe::CallFrame;
use crate::compiler::compiler::Compiler;
use crate::parser::parser::EvalParser;
use crate::compiler::value::Value::Obj;

pub struct VM {
    ip: usize,
    stack_top: usize,
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
    frame: Option<CallFrame>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            ip: 0,
            stack_top: 0,
            stack: vec![],
            frames: Vec::with_capacity(64),
            globals: HashMap::new(),
            frame: None,
        }
    }

    pub fn interpret(&mut self, source: &str) {
        let exprs = EvalParser::parse(source);
        let function = Compiler::compile(exprs);
        println!("{}", function.chunk());

        self.push(Value::Obj(Object::Function(function.clone())));
        self.frames.push(
            CallFrame::new(
                function.clone(),
                function.chunk().code().len(),
                self.stack.clone(),
            )
        );

        self.run()
    }

    fn run(&mut self) {
        while !self.is_at_end() {
            let instruction = Opcode::from(self.read_byte());
            match instruction {
                Opcode::Return => self.ret(),
                Opcode::Constant => self.constant(),
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
                Opcode::DefineGlobal => self.define_global(),
                Opcode::GetGlobal => self.get_global(),
                Opcode::SetGlobal => self.set_global(),
                Opcode::JumpIfFalse => self.jump_if_false(),
                Opcode::Jump => self.jump(),
                Opcode::Pop => { self.pop(); }
                Opcode::GetLocal => self.get_local(),
                Opcode::SetLocal => self.set_local(),
                Opcode::Nil => self.nil(),
            }
        }
    }

    fn ret(&mut self) {
        let popped = self.pop();
        println!("{:?}", popped); // TODO
    }

    fn constant(&mut self) {
        let constant = self.read_constant();
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

    fn define_global(&mut self) {
        let name = value_to_string(self.read_constant());
        let val = self.peek(0);
        self.globals.insert(name, val);
        self.pop();
    }

    fn get_global(&mut self) {
        let name = value_to_string(self.read_constant());
        let value = self.globals.get(&name).cloned();
        self.push(value.unwrap()); // TODO Unwrap???
    }

    fn set_global(&mut self) {
        let name = value_to_string(self.read_constant());
        let peek = self.peek(0);
        if let Some(global) = self.globals.get_mut(&name) {
            *global = peek;
        } else {
            panic!("No global with name: {}", name);
        }
    }

    fn jump_if_false(&mut self) {
        let offset = self.read_short();

        if !bool::from(self.peek(0)) { // TODO
            self.frame_mut().ip += offset as usize;
        }
    }

    fn jump(&mut self) {
        let offset = self.read_short();
        self.frame_mut().ip += offset as usize;
    }

    fn print(&mut self) {
        let popped = self.pop(); // TODO should not pop value of stack because it's an expression
        println!("{:?}", popped); // TODO Implement display for Value enum
    }

    fn get_local(&mut self) {
        let slot = self.read_byte();
        let val = &self.frame().slots[slot as usize];
        self.push(val.clone());
        // self.push(self.stack[slot as usize].clone()); // TODO Clone???
    }

    fn set_local(&mut self) {
        let slot = self.read_byte();
        let peek = self.peek(0);
        if let Some(local) = self.frame_mut().slots.get_mut(slot as usize) {
            *local = peek;
        } else {
            panic!("No local with name: {}", "TODO"); // TODO Name of local
        }
        // if let Some(local) = self.stack.get_mut(slot as usize) {
        //     *local = peek;
        // } else {
        //     panic!("No local with name: {}", "TODO"); // TODO Name of local
        // }
    }

    fn nil(&mut self) {
        self.push(Value::Nil);
    }

    fn read_constant(&mut self) -> Value {
        let constant_index = self.read_byte();
        self.current_chunk_mut().constants()[constant_index as usize].clone()
    }

    fn read_byte(&mut self) -> u8 {
        let index = self.ip;
        let byte = self.current_chunk_mut().code()[index];
        self.ip = self.ip + 1;
        byte
    }

    fn read_short(&mut self) -> u16 {
        self.ip += 2;

        let lo_index = self.ip - 2;
        let hi_index = self.ip - 1;

        let lo = self.current_chunk_mut().code()[lo_index] as u16;
        let hi = self.current_chunk_mut().code()[hi_index] as u16;
        (lo << 8) | hi
    }

    fn is_at_end(&self) -> bool {
        self.ip >= self.current_chunk().code().len()
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

    fn frame(&self) -> &CallFrame {
        self.frames.last().expect("frames to be nonempty")
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().expect("frames to be nonempty")
    }

    fn current_chunk(&self) -> &Chunk {
        &self.frame().function.chunk()
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.frame_mut().function.chunk_mut()
    }
}