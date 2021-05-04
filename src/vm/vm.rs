use crate::compiler::chunk::Chunk;
use crate::compiler::object::{Class, GreenClosure, Instance, Object};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::vm::errors::RuntimeError;
use crate::vm::frame::CallFrame;
use crate::vm::VM;
use std::collections::HashMap;
use crate::vm::obj::Gc;

pub type RunResult<T> = Result<T, RuntimeError>;

impl VM {
    pub(crate) fn run(&mut self) -> RunResult<()> {
        while !self.is_at_end() {
            let instruction = Opcode::from(self.read_byte());
            match instruction {
                Opcode::Constant => self.constant(),
                Opcode::Add => self.add()?,
                Opcode::Subtract => self.subtract()?,
                Opcode::Multiply => self.multiply()?,
                Opcode::Divide => self.divide()?,
                Opcode::Greater => self.greater()?,
                Opcode::Less => self.less()?,
                Opcode::Equal => self.equal()?,
                Opcode::Not => self.not()?,
                Opcode::Negate => self.negate()?,
                Opcode::DefineGlobal => self.define_global()?,
                Opcode::GetGlobal => self.get_global()?,
                Opcode::SetGlobal => self.set_global()?,
                Opcode::GetLocal => self.get_local()?,
                Opcode::SetLocal => self.set_local()?,
                Opcode::GetProperty => self.get_property()?,
                Opcode::SetProperty => self.set_property()?,
                Opcode::Class => self.class(),
                Opcode::Closure => self.closure(),
                Opcode::JumpIfFalse => self.jump_if_false()?,
                Opcode::Jump => self.jump()?,
                Opcode::Loop => self.loop_(),
                Opcode::Call => self.call_instruction(),
                Opcode::NewArray => self.new_array()?,
                Opcode::IndexSubscript => self.index_subscript()?,
                Opcode::StoreSubscript => self.store_subscript()?,
                Opcode::Return => self.ret()?,
                Opcode::Print => self.print()?,
                Opcode::Pop => {
                    self.pop()?;
                }
                Opcode::Nil => self.nil(),
            };
        }

        Ok(())
    }

    fn constant(&mut self) {
        let constant = self.read_constant().clone();
        self.push(constant);
    }

    fn add(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a + b);
        Ok(())
    }

    fn subtract(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a - b);
        Ok(())
    }

    fn multiply(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a * b);
        Ok(())
    }

    fn divide(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a / b);
        Ok(())
    }

    fn equal(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a == b).into());
        Ok(())
    }

    fn greater(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a > b).into());
        Ok(())
    }

    fn less(&mut self) -> RunResult<()> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push((a < b).into());
        Ok(())
    }

    fn not(&mut self) -> RunResult<()> {
        let a = self.pop()?;
        self.push(bool::into(!bool::from(&a)));
        Ok(())
    }

    fn negate(&mut self) -> RunResult<()> {
        let a = self.pop()?;
        self.push(-a);
        Ok(())
    }

    fn ret(&mut self) -> RunResult<()> {
        if let Some(frame) = self.frames.pop() {
            let result = self.pop()?;
            self.stack.truncate(*frame.stack_start());
            self.push(result);
            Ok(())
        } else {
            Err(RuntimeError::ReturnFromTopLevel)
        }
    }

    fn define_global(&mut self) -> RunResult<()> {
        if let Some(value) = self.pop()? {
            let var_name = self.read_string().to_string();
            self.globals.insert(var_name, value);
            Ok(())
        } else {
            Err(RuntimeError::BadStackIndex(10, self.stack.len())) // TODO 10
        }
    }

    fn get_global(&mut self) -> RunResult<()> {
        let name = self.read_constant().as_string().clone();

        if let Some(value) = self.globals.get(&name).cloned() {
            self.push(value);
            Ok(())
        } else {
            Err(RuntimeError::UndefinedGlobal(name))
        }
    }

    fn set_global(&mut self) -> RunResult<()> {
        let name = self.read_constant().as_string().clone();

        if self.globals.contains_key(&name) {
            let value = self.peek()?.clone();
            self.globals.insert(name, value);
            Ok(())
        } else {
            Err(RuntimeError::UndefinedGlobal(name))
        }
    }

    fn jump_if_false(&mut self) -> RunResult<()> {
        let offset = self.read_short();

        if !bool::from(self.peek()?) {
            *self.frame_mut().ip_mut() += offset as usize;
        }
        Ok(())
    }

    fn jump(&mut self) -> RunResult<()> {
        let offset = self.read_short();
        *self.frame_mut().ip_mut() += offset as usize;
        Ok(())
    }

    fn print(&mut self) -> RunResult<()> {
        let popped = self.pop()?;
        println!("{:?}", popped);
        Ok(())
    }

    fn nil(&mut self) {
        self.push(Value::Nil);
    }

    fn get_local(&mut self) -> RunResult<()> {
        let start = *self.frame().stack_start();
        let slot = self.read_byte() as usize;
        let index = start + slot;

        if let Some(value) = self.stack.get(index).cloned() {
            self.stack.push(value);
            Ok(())
        } else {
            Err(RuntimeError::BadStackIndex(index, self.stack.len()))
        }
    }

    fn set_local(&mut self) -> RunResult<()> {
        let value = self.peek()?.clone();
        let start = *self.frame().stack_start();
        let slot = self.read_byte() as usize;
        self.stack[start + slot] = value;
        Ok(())
    }

    fn call_instruction(&mut self) {
        let arity = self.read_byte();
        self.call_value(arity);
    }

    fn closure(&mut self) {
        match self.read_constant().clone() {
            Value::Function(fun) => {
                let closure = GreenClosure::new(fun);
                let clos = self.alloc(closure);
                self.push(Value::Closure(clos));
            }
            _ => todo!(), // TODO
        }
    }

    fn call(&mut self, closure: Gc<GreenClosure>, arity: u8) {
        if arity != *closure.function.arity() {
            panic!( // TODO Error
                    "Expected {} arguments but got {}.",
                    closure.function.arity(),
                    arity
            );
        }

        let last = self.stack.len();
        let frame_start = last - (arity + 1) as usize;

        self.frames.push(CallFrame::new(closure, frame_start));
    }

    pub(crate) fn call_value(&mut self, arity: u8) {
        let frame_start = self.stack.len() - (arity + 1) as usize;
        let callee = self.stack[frame_start].clone();

        match callee {
            Value::Closure(c) => self.call(c, arity),
            Value::Class(c) => {
                let instance = Value::Instance(self.alloc(Instance::new(c)));

                let l = self.stack.len();
                self.stack[l - usize::from(arity) - 1] = instance;
            }
            _ => panic!("Can only call functions"), // TODO Error
        }
    }

    fn loop_(&mut self) {
        let offset = self.read_short();
        *self.frame_mut().ip_mut() -= offset as usize;
    }

    fn new_array(&mut self) -> RunResult<()> {
        // Stack before: [item1, item2, ..., itemN] and after: [array]
        let mut array = vec![];
        let item_count = self.read_byte();

        // Move items from stack to array
        for _ in 0..item_count {
            array.push(self.pop()?);
        }

        array.reverse();

        self.push(Value::Array(array));
        Ok(())
    }

    fn index_subscript(&mut self) -> RunResult<()> {
        // Stack before: [array, index] and after: [index(array, index)]
        let index = self.pop()?.as_number();
        let array = self.pop()?.as_array();

        // Stack before: [array, index] and after: [index(array, index)]
        let result = array[index as usize].clone();
        self.push(result);
        Ok(())
    }

    fn store_subscript(&mut self) -> RunResult<()> {
        // Stack before: [array, index, item] and after: [item]
        let item = self.pop()?;
        let index = self.pop()?.as_number();
        let mut array = self.pop()?.as_array();

        // Stack before: [array, index] and after: [index(array, index)]
        array[index as usize] = item;
        let result = array.clone();
        self.push(Value::Array(result));

        Ok(())
    }

    fn class(&mut self) {
        let name = self.read_constant().as_string();
        let cls = Class::new(name.clone());
        let class = Value::Class(self.alloc(cls));
        self.push(class);
    }

    fn get_property(&mut self) -> RunResult<()> {
        if !self.peek()?.is_instance() {
            panic!("Only instances have properties."); // TODO Error
        }

        match self.stack.pop() {
            Some(Value::Instance(i)) => {
                let name = self.read_string();

                if let Some(value) = i.fields.get(name) {
                    self.push(value.clone());
                    Ok(())
                } else {
                    Err(RuntimeError::UndefinedProperty(name.to_string()))
                }
            }
            _ => todo!(), // TODO
        }
    }

    fn set_property(&mut self) -> RunResult<()> {
        // Stack before: [instance, value, property] and after: [index(array, index)] TODO After
        let value = self.pop()?;

        let mut instance = self.pop()?.as_instance()?;
        let property = self.read_string();

        instance.fields.insert(property.to_string(), value.clone());
        self.push(value);

        Ok(())
    }

    fn read_string(&mut self) -> &String {
        self.read_constant().as_string()
    }

    fn read_constant(&mut self) -> &Value {
        let constant_index = self.read_byte();
        self.current_chunk().read_constant(constant_index.into())
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

    pub(crate) fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn peek(&mut self) -> RunResult<&Value> {
        self.stack.last().ok_or(RuntimeError::StackEmpty)
    }

    fn peek_offset(&mut self, offset: usize) -> Value {
        let index = self.stack.len() - 1 - offset; // TODO Error
        self.stack[index as usize].clone()
    }

    fn pop(&mut self) -> RunResult<Value> {
        self.stack.pop().ok_or(RuntimeError::StackEmpty)
    }

    fn frame(&self) -> &CallFrame {
        self.frames.last().expect("frames to be nonempty") // TODO Error
    }

    fn frame_mut(&mut self) -> &mut CallFrame {
        self.frames.last_mut().expect("frames to be nonempty") // TODO Error
    }

    fn current_chunk(&self) -> &Chunk {
        &self.frame().closure().function.chunk()
    }

    fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.frame_mut().closure_mut().function.chunk_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::object::GreenFunction;

    #[test]
    fn it_works() {
        // let mut vm = VM::new();
        //
        // let mut function = GreenFunction::new();
        // let chunk = function.chunk_mut();
        //
        // chunk.write(Opcode::Class, 0);
        //
        // chunk.add_constant(Value::String("Point".to_string()))
        // chunk.write(Opcode::DefineGlobal, 0);
        //
        // let closure = GreenClosure::new(function);
        // vm.push(Value::closure(closure.clone()));
        // vm.call_value(Value::closure(closure.clone()), 0);
        //
        // vm.run().unwrap();
    }
}
