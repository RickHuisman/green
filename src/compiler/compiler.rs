use crate::compiler::chunk::Chunk;
use crate::compiler::instance::CompilerInstance;
use crate::compiler::local::Local;
use crate::compiler::module_resolver::get_module_ast;
use crate::compiler::object::{GreenFunction, GreenFunctionType};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::syntax::expr::{
    BinaryExpr, BinaryOperator, BlockExpr, CallExpr, Compile, Expr, ExprKind, FunctionExpr,
    GroupingExpr, IfElseExpr, IfExpr, ImportExpr, LiteralExpr, ReturnExpr, UnaryExpr,
    UnaryOperator, VarAssignExpr, VarGetExpr, VarSetExpr, Variable, WhileExpr,
};
use crate::syntax::parser::ModuleAst;

pub struct Compiler {
    pub(crate) current: CompilerInstance,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            current: CompilerInstance::new(GreenFunctionType::Script),
        }
    }

    pub fn compile_module(module: ModuleAst) -> GreenFunction {
        let mut compiler = Compiler::new();

        for expr in module.exprs() {
            compiler.compile_expr(expr);
        }

        compiler.end_compiler()
    }

    pub fn compile_expr(&mut self, expr: &Expr) {
        expr.node.compile(self);
    }

    // var x = 10
    pub(crate) fn compile_declare_var(&mut self, var: &Variable) {
        if *self.current.scope_depth() == 0 as isize {
            return;
        }

        for local in self.current.locals() {
            if *local.depth() != -1 as isize && local.depth() < &self.current.scope_depth() {
                break;
            }

            if var.name == *local.name() {
                panic!("Already a variable called {} in this scope.", var.name);
            }
        }

        self.add_local(var.name.to_string());
        self.mark_initialized();
    }

    pub(crate) fn compile_define_var(&mut self, var: &Variable) {
        if *self.current.scope_depth() > 0 {
            self.mark_initialized();
            return;
        }

        self.emit(Opcode::DefineGlobal);
        let constant_id = self
            .current_chunk()
            .add_constant(Value::string(var.name.clone()));
        self.emit_byte(constant_id);
    }

    pub(crate) fn emit_loop(&mut self, loop_start: usize) {
        self.emit(Opcode::Loop);

        let chunk = self.current_chunk();
        let sub = chunk.code().len() - loop_start + 2;

        let lo = ((sub >> 8) & 0xff) as u8;
        let hi = (sub & 0xff) as u8;

        self.emit_byte(lo);
        self.emit_byte(hi);
    }

    pub(crate) fn emit_jump(&mut self, instruction: Opcode) -> usize {
        self.emit(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        return self.current_chunk().code().len() - 2;
    }

    pub(crate) fn patch_jump(&mut self, offset: usize) {
        // -2 to adjust for the bytecode for the jump offset itself.
        let jump = self.current_chunk().code().len() - offset - 2;

        self.current_chunk().code_mut()[offset] = ((jump >> 8) & 0xff) as u8;
        self.current_chunk().code_mut()[offset + 1] = (jump & 0xff) as u8;
    }

    fn compile_literal(&mut self, literal: &LiteralExpr) {
        match literal {
            LiteralExpr::Number(n) => self.emit_constant(Value::Number(*n)),
            LiteralExpr::String(s) => self.emit_string(&s),
            LiteralExpr::True => self.emit_constant(Value::True),
            LiteralExpr::False => self.emit_constant(Value::False),
            _ => todo!(), // TODO NilLiteral
        }
    }

    pub(crate) fn resolve_local(&self, name: &String) -> isize {
        for (i, local) in self.current.locals().iter().enumerate() {
            if *name == *local.name() {
                if *local.depth() == -1 {
                    panic!(
                        "Can't read local variable {} in it's own initializer.",
                        name
                    );
                }

                return i as isize;
            }
        }

        -1
    }

    fn add_local(&mut self, name: String) {
        let local = Local::new(name, -1);
        self.current.locals_mut().push(local);
    }

    fn mark_initialized(&mut self) {
        if *self.current.scope_depth() == 0 {
            return;
        }

        let index = &self.current.locals().len() - 1;
        *self.current.locals_mut()[index].depth_mut() = *self.current.scope_depth();
    }

    pub(crate) fn begin_scope(&mut self) {
        *self.current.scope_depth_mut() += 1;
    }

    pub(crate) fn end_scope(&mut self) {
        *self.current.scope_depth_mut() -= 1;

        while self.current.locals().len() > 0
            && self.current.locals()[self.current.locals().len() - 1].depth()
                > self.current.scope_depth()
        {
            self.emit(Opcode::Pop);
            self.current.locals_mut().pop();
        }
    }

    pub(crate) fn end_compiler(&mut self) -> GreenFunction {
        self.emit_return();
        let fun_copy = self.current.function().clone();

        println!("{}", self.current_chunk());

        if let Some(enclosing) = *self.current.enclosing().clone() {
            self.current = enclosing;
        }

        fun_copy
    }

    pub(crate) fn current_chunk(&mut self) -> &mut Chunk {
        self.current.function_mut().chunk_mut()
    }

    pub(crate) fn emit_return(&mut self) {
        self.emit(Opcode::Nil);
        self.emit(Opcode::Return);
    }

    pub(crate) fn emit_string(&mut self, s: &str) {
        self.emit_constant(Value::Obj(s.into()));
    }

    pub(crate) fn emit_constant(&mut self, value: Value) {
        let constant = self.current_chunk().add_constant(value);
        self.emit(Opcode::Constant);
        self.emit_byte(constant);
    }

    pub(crate) fn emit(&mut self, opcode: Opcode) {
        self.current_chunk().write(opcode, 123); // TODO Line
    }

    pub(crate) fn emit_byte(&mut self, byte: u8) {
        self.current_chunk().write_byte(byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parser::GreenParser;

    fn parse_source(str: &str) -> ModuleAst {
        GreenParser::parse(str).unwrap()
    }

    #[test]
    fn compile_for() {
        let input = r#"
        for x in 1 to 5 do
        end
        "#;
        let module = parse_source(input);
        let chunk = Compiler::compile_module(module);
    }
}
