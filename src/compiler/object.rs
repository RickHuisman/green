use crate::compiler::chunk::Chunk;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Function(EvalFunction),
}

#[derive(Debug, Clone)]
pub enum EvalFunctionType {
    Function,
    Script,
}

#[derive(Debug, Clone)]
pub struct EvalFunction {
    name: String,
    chunk: Chunk,
    arity: u8,
}

impl EvalFunction {
    pub fn new(name: String) -> Self {
        EvalFunction { name, chunk: Chunk::new(), arity: 0 }
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
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