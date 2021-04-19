use crate::compiler::local::Local;
use crate::compiler::object::{GreenFunction, GreenFunctionType};

#[derive(Debug, Clone)]
pub struct CompilerInstance {
    function: GreenFunction,
    function_type: GreenFunctionType,
    locals: Vec<Local>,
    scope_depth: isize,
    enclosing: Box<Option<CompilerInstance>>,
}

impl CompilerInstance {
    pub fn new(function_type: GreenFunctionType) -> Self {
        let mut compiler = CompilerInstance {
            function: GreenFunction::new(),
            function_type,
            locals: Vec::with_capacity(u8::MAX as usize),
            scope_depth: 0,
            enclosing: Box::new(None),
        };
        compiler.locals.push(Local::new("".to_string(), 0));

        compiler
    }

    pub fn function(&self) -> &GreenFunction {
        &self.function
    }

    pub fn function_mut(&mut self) -> &mut GreenFunction {
        &mut self.function
    }

    pub fn function_type(&self) -> &GreenFunctionType {
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
