use crate::compiler::chunk::Chunk;
use crate::compiler::compiler::Compiler;
use crate::compiler::object::{EvalClosure, Object};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::{value_to_string, Value};
use crate::syntax::parser::{EvalParser, ModuleAst};
use crate::vm::callframe::CallFrame;
use std::collections::HashMap;
use std::process::exit;

pub struct VM {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: vec![],
            frames: Vec::with_capacity(64),
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) {
        let module = match EvalParser::parse(source) {
            Ok(m) => m,
            Err(err) => {
                println!("{}", err);
                exit(1);
            }
        };
        let function = Compiler::compile_module(module);

        let closure = EvalClosure::new(function);
        self.push(Value::closure(closure.clone()));
        self.call_value(Value::closure(closure.clone()), 0);

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
                Opcode::Pop => {
                    self.pop();
                }
                Opcode::GetLocal => self.get_local(),
                Opcode::SetLocal => self.set_local(),
                Opcode::Nil => self.nil(),
                Opcode::Call => self.call_instruction(),
                Opcode::Closure => self.closure(),
                Opcode::Loop => self.loop_(),
            }
        }
    }

    fn ret(&mut self) {
        if let Some(frame) = self.frames.pop() {
            let result = self.pop();
            self.stack.truncate(*frame.stack_start());
            self.push(result);
        } else {
            panic!("Cannot return from top-level.");
        }
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
        self.push(bool::into(!bool::from(a)));
    }

    fn negate(&mut self) {
        let a = self.pop();
        self.push(-a);
    }

    fn define_global(&mut self) {
        let name = value_to_string(self.read_constant());
        let val = self.peek();
        self.globals.insert(name, val);
        self.pop();
    }

    fn get_global(&mut self) {
        let name = value_to_string(self.read_constant());
        let value = self.globals.get(&name).cloned();
        self.push(value.unwrap());
    }

    fn set_global(&mut self) {
        let name = value_to_string(self.read_constant());
        let peek = self.peek();
        if let Some(global) = self.globals.get_mut(&name) {
            *global = peek;
        } else {
            panic!("No global with name: {}", name);
        }
    }

    fn jump_if_false(&mut self) {
        let offset = self.read_short();

        if !bool::from(self.peek()) {
            // TODO
            *self.frame_mut().ip_mut() += offset as usize;
        }
    }

    fn jump(&mut self) {
        let offset = self.read_short();
        *self.frame_mut().ip_mut() += offset as usize;
    }

    fn print(&mut self) {
        let popped = self.pop(); // TODO should not pop value of stack because it's an expression
        println!("{}", popped);
    }

    fn get_local(&mut self) {
        let start = *self.frame().stack_start();
        let idx = self.read_byte() as usize;
        let val = self.stack[start + idx].clone(); // Clone???
        self.push(val);
    }

    fn set_local(&mut self) {
        // We peek because we would just push it back after
        // the assignment occurs.
        let val = self.peek();
        let start = *self.frame().stack_start();
        let idx = self.read_byte() as usize;
        self.stack[start + idx] = val;
    }

    fn nil(&mut self) {
        self.push(Value::Nil);
    }

    fn call_instruction(&mut self) {
        let arity = self.read_byte();
        let frame_start = self.stack.len() - (arity + 1) as usize;
        let callee = self.stack[frame_start].clone();

        if !self.call_value(callee, arity) {
            panic!("TODO");
        }
    }

    fn closure(&mut self) {
        let function = self.read_constant(); // TODO Convert to function

        // FIXME
        match function {
            Value::Obj(obj) => match obj {
                Object::Function(fun) => {
                    let closure = EvalClosure::new(fun);
                    self.push(Value::Obj(Object::Closure(closure)));
                }
                _ => {
                    println!("{:?}", obj);
                    todo!()
                }
            },
            _ => todo!(),
        }
    }

    fn call(&mut self, closure: EvalClosure, arity: u8) -> bool {
        if arity != *closure.function.arity() {
            panic!(
                "Expected {} arguments but got {}.",
                closure.function.arity(),
                arity
            );
        }

        let last = self.stack.len();
        let frame_start = last - (arity + 1) as usize;

        self.frames.push(CallFrame::new(closure, frame_start));

        true
    }

    fn call_value(&mut self, callee: Value, arity: u8) -> bool {
        // Check if callee is obj
        match callee {
            Value::Obj(obj) => {
                match obj {
                    Object::Closure(c) => self.call(c, arity),
                    // Object::Function(fun) => self.call(fun, arity),
                    _ => panic!("Can only call functions"),
                }
            }
            _ => panic!("Can only call functions"),
        }
    }

    fn loop_(&mut self) {
        let offset = self.read_short();
        *self.frame_mut().ip_mut() -= offset as usize;
    }

    fn read_constant(&mut self) -> Value {
        let constant_index = self.read_byte();
        self.current_chunk_mut().constants()[constant_index as usize].clone()
    }

    fn read_byte(&mut self) -> u8 {
        let index = *self.frame().ip();
        let byte = self.current_chunk_mut().code()[index];
        *self.frame_mut().ip_mut() += 1;
        byte
    }

    fn read_short(&mut self) -> u16 {
        *self.frame_mut().ip_mut() += 2;

        let lo_index = self.frame().ip() - 2;
        let hi_index = self.frame().ip() - 1;

        let lo = self.current_chunk_mut().code()[lo_index] as u16;
        let hi = self.current_chunk_mut().code()[hi_index] as u16;
        (lo << 8) | hi
    }

    fn is_at_end(&self) -> bool {
        self.frames.is_empty()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn peek(&mut self) -> Value {
        self.stack.last().expect("stack to be nonempty").clone()
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Failed to pop value from stack")
    }

    fn frame(&self) -> &CallFrame {
        self.frames.last().expect("frames to be nonempty")
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().expect("frames to be nonempty")
    }

    fn current_chunk(&self) -> &Chunk {
        &self.frame().closure().function.chunk()
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.frame_mut().closure_mut().function.chunk_mut()
    }
}
