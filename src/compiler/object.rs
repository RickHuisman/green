use crate::compiler::chunk::Chunk;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Closure(EvalClosure),
    Function(EvalFunction),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Closure(c) => write!(f, "<fn {}>", c.function.name),
            Object::Function(fun) => write!(f, "<fn {}>", fun.name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EvalFunctionType {
    Closure,
    Function,
    Script,
}

#[derive(Debug, Clone)]
pub struct EvalClosure {
    pub function: EvalFunction,
}

impl EvalClosure {
    pub fn new(function: EvalFunction) -> EvalClosure {
        EvalClosure { function }
    }
}

#[derive(Debug, Clone)]
pub struct EvalFunction {
    name: String,
    chunk: Chunk,
    arity: u8,
}

impl EvalFunction {
    pub fn new() -> Self {
        EvalFunction { name: "".to_string(), chunk: Chunk::new(), arity: 0 }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub fn arity(&self) -> &u8 {
        &self.arity
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    pub fn arity_mut(&mut self) -> &mut u8 {
        &mut self.arity
    }
}

impl Into<Object> for &str {
    fn into(self) -> Object {
        Object::String(self.to_string()) // TODO Object(String) should be Object(&str)
    }
}

impl Into<Object> for String {
    fn into(self) -> Object {
        Object::String(self)
    }
}