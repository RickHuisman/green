use crate::compiler::compiler::Compiler;
use crate::compiler::object::GreenClosure;
use crate::compiler::value::Value;
use crate::syntax::parser::GreenParser;
use crate::vm::frame::CallFrame;
use std::collections::HashMap;
use std::process::exit;
use crate::vm::obj::Gc;

pub mod errors;
mod frame;
mod run;
pub mod vm;
pub mod gc;
pub mod obj;

pub struct VM {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
}

impl<'source> VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::with_capacity(256),
            frames: Vec::with_capacity(256),
            globals: HashMap::new(),
        }
    }

    pub fn interpret<T: AsRef<str> + 'source>(&mut self, source: T) {
        // TODO Return errors
        let module = match GreenParser::parse(source.as_ref()) {
            Ok(m) => m,
            Err(err) => {
                println!("{}", err);
                exit(1);
            }
        };
        let function = Compiler::compile(module);

        let closure = self.alloc(GreenClosure::new(Gc::new(function)).clone());
        self.push(Value::Closure(closure));
        self.call_value(0);

        self.run().unwrap();
    }
}
