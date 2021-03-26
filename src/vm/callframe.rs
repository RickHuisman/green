use crate::compiler::object::EvalFunction;
use crate::compiler::value::Value;

#[derive(Clone)]
pub struct CallFrame {
    pub function: EvalFunction,
    pub ip: usize,
    pub slots: Vec<Value>, // TODO Vec or array
}

impl CallFrame {
    pub fn new(function: EvalFunction, ip: usize, slots: Vec<Value>) -> Self {
        CallFrame {
            function,
            ip,
            slots
        }
    }
}