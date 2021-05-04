use crate::compiler::chunk::Chunk;
use crate::compiler::value::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::vm::obj::Gc;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Array(Vec<Value>), // TODO u32? Vec?
    Closure(GreenClosure),
    Function(GreenFunction),
    Class(Class),
    Instance(Instance),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Array(a) => write!(f, "{:?}", a),
            Object::Closure(c) => write!(f, "<fn {}>", c.function.name),
            Object::Function(fun) => write!(f, "<fn {}>", fun.name),
            Object::Class(cls) => write!(f, "{}", cls.name),
            Object::Instance(i) => write!(f, "{} instance", i.class.name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GreenFunctionType {
    Closure,
    Function,
    Script,
}

#[derive(Debug, Clone)]
pub struct GreenClosure {
    pub function: Gc<GreenFunction>,
}

impl GreenClosure {
    pub fn new(function: Gc<GreenFunction>) -> GreenClosure {
        GreenClosure { function }
    }
}

#[derive(Debug, Clone)]
pub struct GreenFunction {
    name: String,
    chunk: Chunk,
    arity: u8,
}

impl GreenFunction {
    pub fn new() -> Self {
        GreenFunction {
            name: "".to_string(),
            chunk: Chunk::new(),
            arity: 0,
        }
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

impl fmt::Display for GreenFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<fun {}/{}>", self.name, self.arity)
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

#[derive(Debug, Clone)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Gc<Class>,
    pub fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Gc<Class>) -> Self {
        Instance {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get_property(&self, name: &str) -> Option<Value> {
        self.fields.get(name).cloned()
    }

    pub fn set_property(&mut self, property: &str, value: Value) {
        if let Some(v) = self.fields.get_mut(property) {
            *v = value;
        } else {
            self.fields.insert(property.to_string(), value);
        }
    }
}
