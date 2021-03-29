use crate::compiler::object::EvalClosure;

#[derive(Clone)]
pub struct CallFrame {
    pub closure: EvalClosure,
    pub ip: usize,
    pub stack_start: usize,
}

impl CallFrame {
    pub fn new(closure: EvalClosure, stack_start: usize) -> Self {
        CallFrame {
            closure,
            ip: 0,
            stack_start,
        }
    }
}