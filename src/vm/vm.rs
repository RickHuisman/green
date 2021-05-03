use crate::compiler::chunk::Chunk;
use crate::compiler::compiler::Compiler;
use crate::compiler::object::{GreenClosure, Object, Class, Instance};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::syntax::parser::{GreenParser, ModuleAst};
use crate::vm::callframe::CallFrame;
use std::collections::HashMap;
use std::process::exit;
use crate::compiler::value::Value::Obj;

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

    pub fn interpret(&mut self, source: &str) { // TODO Result
        let module = match GreenParser::parse(source) {
            Ok(m) => m,
            Err(err) => {
                println!("{}", err);
                exit(1);
            }
        };
        let function = Compiler::compile_module(module);

        let closure = GreenClosure::new(function);
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
                Opcode::Pop => { self.pop(); }
                Opcode::GetLocal => self.get_local(),
                Opcode::SetLocal => self.set_local(),
                Opcode::Nil => self.nil(),
                Opcode::Call => self.call_instruction(),
                Opcode::Closure => self.closure(),
                Opcode::Loop => self.loop_(),
                Opcode::NewArray => self.new_array(),
                Opcode::IndexSubscript => self.index_subscript(),
                Opcode::StoreSubscript => self.store_subscript(),
                Opcode::Class => self.class(),
                Opcode::GetProperty => self.get_property(),
                Opcode::SetProperty => self.set_property(),
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
        let name = self.read_constant().as_string();
        let value = self.peek();
        self.globals.insert(name, value);
        self.pop();
    }

    fn get_global(&mut self) {
        let name = self.read_constant().as_string();
        let value = self.globals.get(&name).cloned();

        self.push(value.unwrap());
    }

    fn set_global(&mut self) {
        let name = self.read_constant().as_string();
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
            *self.frame_mut().ip_mut() += offset as usize;
        }
    }

    fn jump(&mut self) {
        let offset = self.read_short();
        *self.frame_mut().ip_mut() += offset as usize;
    }

    fn print(&mut self) {
        let popped = self.pop();
        println!("{}", popped);
    }

    fn nil(&mut self) {
        self.push(Value::Nil);
    }

    fn get_local(&mut self) {
        let start = *self.frame().stack_start();
        let slot = self.read_byte() as usize;
        let value = self.stack[start + slot].clone();
        self.push(value);
    }

    fn set_local(&mut self) {
        let value = self.peek();
        let start = *self.frame().stack_start();
        let slot = self.read_byte() as usize;
        self.stack[start + slot] = value;
    }

    fn call_instruction(&mut self) {
        let arity = self.read_byte();
        let frame_start = self.stack.len() - (arity + 1) as usize;
        let callee = self.stack[frame_start].clone();

        self.call_value(callee, arity);
    }

    fn closure(&mut self) {
        let fun = self.read_constant().as_function();
        let closure = GreenClosure::new(fun);

        self.push(Value::Obj(Object::Closure(closure)));
    }

    fn call(&mut self, closure: GreenClosure, arity: u8) {
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
    }

    fn call_value(&mut self, callee: Value, arity: u8) {
        match callee.as_object() {
            Object::Closure(c) => {
                self.call(c, arity);
            }
            Object::Class(c) => {
                let instance = Value::Obj(Object::Instance(Instance::new(c)));

                // let frame_start = last - (arity + 1) as usize;

                let stack_top = self.stack.len() - (arity + 1) as usize;
                // let stack_top = 2 as usize;
                self.push(instance);
                // self.stack[stack_top] = instance;
            }
            _ => panic!("Can only call functions"),
        }
    }

    fn loop_(&mut self) {
        let offset = self.read_short();
        *self.frame_mut().ip_mut() -= offset as usize;
    }

    fn new_array(&mut self) {
        // Stack before: [item1, item2, ..., itemN] and after: [array]
        let mut array = vec![];
        let mut item_count = self.read_byte();

        // Move items from stack to array
        for _ in 0..item_count {
            array.push(self.pop());
        }

        array.reverse();

        self.push(Value::Obj(Object::Array(array)))
    }

    fn index_subscript(&mut self) {
        // Stack before: [array, index] and after: [index(array, index)]
        let index = self.pop().as_number();
        let array = self.pop().as_array();

        // Stack before: [array, index] and after: [index(array, index)]
        let result = array[index as usize].clone();
        self.push(result);
    }

    fn store_subscript(&mut self) {
        // Stack before: [array, index, item] and after: [item]
        let item = self.pop();
        let index = self.pop().as_number();
        let mut array = self.pop().as_array();

        // Stack before: [array, index] and after: [index(array, index)]
        array[index as usize] = item;
        let result = array.clone();
        self.push(Value::Obj(Object::Array(result)));
    }

    fn class(&mut self) {
        let name = self.read_constant().as_string();
        let class = Value::Obj(Object::Class(Class::new(name)));
        self.push(class);
    }

    fn get_property(&mut self) {
        if !self.peek().is_instance() {
            panic!("Only instances have properties.");
        }

        let instance = self.pop().as_instance();
        let name = self.read_constant().as_string();

        if let Some(value) = instance.fields.get(&name) {
            self.push(value.clone());
        } else {
            panic!("Undefined property '{}'.", name);
        }
    }

    fn set_property(&mut self) {
        let value = self.stack.pop().unwrap();

        // let mut instance = self.stack.pop().unwrap().as_instance();
        match self.stack.pop() {
            None => {}
            Some(Value::Obj(Object::Instance(mut i))) => {
                let var_str = self.read_string();

                i.fields.insert(var_str.to_string(), value.clone());
                self.push(value);
            }
            _ => todo!(),
        }

        // Stack before: [instance, value, property] and after: [index(array, index)] TODO After
        // let property = self.read_constant().as_string();
        //
        // let mut instance = self.peek_offset(1).as_instance();
        // let value = self.peek();
        //
        // instance.set_property(&property, value);
        // println!("{:?}", &instance);
        //
        // let value2 = self.pop();
        // self.pop();
        // self.push(value2);
    }

    fn read_string(&mut self) -> String {
       self.read_constant().as_string()
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

    fn peek_offset(&mut self, offset: usize) -> Value {
        let index = self.stack.len() - 1 - offset;
        self.stack[index as usize].clone()
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
