use crate::compiler::object::EvalFunction;
use crate::compiler::value::Value;

#[derive(Clone)]
pub struct CallFrame {
    pub function: EvalFunction,
    pub ip: usize,
    pub stack_start: usize,
    // pub slots: Vec<Value>, // TODO Vec or array
}

impl CallFrame {
    pub fn new(function: EvalFunction, stack_start: usize) -> Self {
        CallFrame {
            function,
            ip: 0,
            stack_start,
        }
    }
}