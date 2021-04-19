use crate::compiler::object::GreenClosure;

#[derive(Clone)]
pub struct CallFrame {
    closure: GreenClosure,
    ip: usize,
    stack_start: usize,
}

impl CallFrame {
    pub fn new(closure: GreenClosure, stack_start: usize) -> Self {
        CallFrame {
            closure,
            ip: 0,
            stack_start,
        }
    }

    pub fn closure(&self) -> &GreenClosure {
        &self.closure
    }

    pub fn closure_mut(&mut self) -> &mut GreenClosure {
        &mut self.closure
    }

    pub fn ip(&self) -> &usize {
        &self.ip
    }

    pub fn ip_mut(&mut self) -> &mut usize {
        &mut self.ip
    }

    pub fn stack_start(&self) -> &usize {
        &self.stack_start
    }
}
