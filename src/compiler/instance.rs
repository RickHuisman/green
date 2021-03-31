use crate::compiler::object::{EvalFunction, EvalFunctionType, EvalClosure};
use crate::compiler::local::Local;

#[derive(Debug, Clone)]
pub struct CompilerInstance {
    function: EvalFunction,
    function_type: EvalFunctionType,
    locals: Vec<Local>,
    scope_depth: isize,
    enclosing: Box<Option<CompilerInstance>>,
}

impl CompilerInstance {
    pub fn new(function_type: EvalFunctionType) -> Self {
        let mut compiler = CompilerInstance {
            function: EvalFunction::new(),
            function_type,
            locals: Vec::with_capacity(u8::MAX as usize),
            scope_depth: 0,
            enclosing: Box::new(None),
        };
        compiler.locals.push(Local::new("".to_string(), 0));

        compiler
    }

    pub fn function(&self) -> &EvalFunction {
        &self.function
    }

    pub fn function_mut(&mut self) -> &mut EvalFunction {
        &mut self.function
    }

    pub fn function_type(&self) -> &EvalFunctionType {
        &self.function_type
    }

    pub fn locals(&self) -> &Vec<Local> {
        &self.locals
    }

    pub fn locals_mut(&mut self) -> &mut Vec<Local> {
        &mut self.locals
    }

    pub fn scope_depth(&self) -> &isize {
        &self.scope_depth
    }

    pub fn scope_depth_mut(&mut self) -> &mut isize {
        &mut self.scope_depth
    }

    pub fn enclosing(&self) -> &Box<Option<CompilerInstance>> {
        &self.enclosing
    }

    pub fn enclosing_mut(&mut self) -> &mut Box<Option<CompilerInstance>> {
        &mut self.enclosing
    }
}