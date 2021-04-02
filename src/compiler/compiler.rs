use crate::compiler::chunk::Chunk;
use crate::compiler::instance::CompilerInstance;
use crate::compiler::local::Local;
use crate::compiler::module_resolver::get_module_ast;
use crate::compiler::object::{EvalFunction, EvalFunctionType};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::syntax::expr::{
    BinaryExpr, BinaryOperator, BlockExpr, CallExpr, Expr, ExprKind, ForExpr, FunctionExpr,
    GroupingExpr, IfElseExpr, IfExpr, ImportExpr, LiteralExpr, ReturnExpr, UnaryExpr,
    UnaryOperator, VarAssignExpr, VarGetExpr, VarSetExpr, Variable, WhileExpr,
};
use crate::syntax::parser::ModuleAst;

pub struct Compiler {
    current: CompilerInstance,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            current: CompilerInstance::new(EvalFunctionType::Script),
        }
    }

    pub fn compile_module(module: ModuleAst) -> EvalFunction {
        let mut compiler = Compiler::new();

        for expr in module.exprs() {
            compiler.compile_expr(expr);
        }

        compiler.end_compiler()
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match &*expr.node {
            ExprKind::Import(import) => self.compile_import(import),
            ExprKind::Literal(literal) => self.compile_literal(literal),
            ExprKind::Binary(binary) => self.compile_binary(binary),
            ExprKind::Unary(unary) => self.compile_unary(unary),
            ExprKind::Block(block) => self.compile_block(block),
            ExprKind::Print(print) => self.compile_print(print),
            ExprKind::Grouping(grouping) => self.compile_grouping(grouping),
            ExprKind::VarAssign(var) => self.compile_var_expr(var),
            ExprKind::VarSet(var) => self.compile_set_var(var),
            ExprKind::VarGet(var) => self.compile_get_var(var),
            ExprKind::If(if_expr) => self.compile_if(if_expr),
            ExprKind::IfElse(if_else_expr) => self.compile_if_else(if_else_expr),
            ExprKind::Function(function) => self.compile_function(function),
            ExprKind::Call(call) => self.compile_call(call),
            ExprKind::Return(ret_expr) => self.compile_return(ret_expr),
            ExprKind::For(for_expr) => self.compile_for(for_expr),
            ExprKind::While(while_expr) => self.compile_while(while_expr),
        }
    }

    fn compile_import(&mut self, import: &ImportExpr) {
        let module = get_module_ast(&import.module).unwrap();

        // TODO Only compile top level expressions
        for expr in module.exprs() {
            self.compile_expr(expr);
        }
    }

    fn compile_binary(&mut self, binary: &BinaryExpr) {
        self.compile_expr(&binary.lhs);
        self.compile_expr(&binary.rhs);

        match binary.operator {
            BinaryOperator::Add => self.emit(Opcode::Add),
            BinaryOperator::Subtract => self.emit(Opcode::Subtract),
            BinaryOperator::Multiply => self.emit(Opcode::Multiply),
            BinaryOperator::Divide => self.emit(Opcode::Divide),
            BinaryOperator::Equal => self.emit(Opcode::Equal),
            BinaryOperator::BangEqual => {
                self.emit(Opcode::Equal);
                self.emit(Opcode::Not);
            }
            BinaryOperator::GreaterThan => self.emit(Opcode::Greater),
            BinaryOperator::GreaterThanEqual => {
                self.emit(Opcode::Less);
                self.emit(Opcode::Not);
            }
            BinaryOperator::LessThan => self.emit(Opcode::Less),
            BinaryOperator::LessThanEqual => {
                self.emit(Opcode::Greater);
                self.emit(Opcode::Not);
            }
        }
    }

    fn compile_unary(&mut self, unary: &UnaryExpr) {
        self.compile_expr(&unary.expr);

        let op = match unary.operator {
            UnaryOperator::Negate => Opcode::Negate,
            UnaryOperator::Not => Opcode::Not,
        };
        self.emit(op);
    }

    fn compile_block(&mut self, block: &BlockExpr) {
        self.begin_scope();
        for expr in &block.exprs {
            self.compile_expr(expr);
        }
        self.end_scope();
    }

    fn compile_print(&mut self, expr: &Expr) {
        self.compile_expr(&expr);
        self.emit(Opcode::Print);
    }

    fn compile_grouping(&mut self, grouping: &GroupingExpr) {
        self.compile_expr(&grouping.expr);
    }

    fn compile_var_expr(&mut self, var: &VarAssignExpr) {
        // TODO Check if initialized -> if not init with nil
        self.compile_expr(&var.initializer);

        if *self.current.scope_depth() > 0 as isize {
            // Local
            self.compile_declare_var(&var.variable);
        } else {
            // Global
            self.compile_define_var(&var.variable);
        }
    }

    // var x = 10
    fn compile_declare_var(&mut self, var: &Variable) {
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

    fn compile_define_var(&mut self, var: &Variable) {
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

    // x = 10
    fn compile_set_var(&mut self, var: &VarSetExpr) {
        self.compile_expr(&var.initializer);

        let var_name = &var.variable.name;
        let arg = self.resolve_local(var_name);
        if arg != -1 {
            // Local
            self.emit(Opcode::SetLocal);
            self.emit_byte(arg as u8);
        } else {
            // Global
            self.emit(Opcode::SetGlobal);
            let str_obj = Value::string(var_name.clone());
            let constant_id = self.current_chunk().add_constant(str_obj);
            self.emit_byte(constant_id);
        }
    }

    // print(x)
    fn compile_get_var(&mut self, var: &VarGetExpr) {
        let var_name = &var.variable.name;
        let arg = self.resolve_local(var_name);
        if arg != -1 {
            // Local
            self.emit(Opcode::GetLocal);
            self.emit_byte(arg as u8);
        } else {
            // Global
            self.emit(Opcode::GetGlobal);
            let str_obj = Value::string(var_name.clone());
            let constant_id = self.current_chunk().add_constant(str_obj);
            self.emit_byte(constant_id);
        }
    }

    fn compile_if(&mut self, if_expr: &IfExpr) {
        self.compile_expr(&if_expr.condition);

        // Jump to else clause if false
        let then_jump = self.emit_jump(Opcode::JumpIfFalse);
        self.emit(Opcode::Pop);

        for expr in &if_expr.then_clause {
            self.compile_expr(expr);
        }

        let else_jump = self.emit_jump(Opcode::Jump);

        self.patch_jump(then_jump);
        self.emit(Opcode::Pop);

        self.patch_jump(else_jump);
    }

    fn compile_if_else(&mut self, if_else_expr: &IfElseExpr) {
        self.compile_expr(&if_else_expr.condition);

        // Jump to else clause if false
        let then_jump = self.emit_jump(Opcode::JumpIfFalse);
        self.emit(Opcode::Pop);

        for expr in &if_else_expr.then_clause.exprs {
            self.compile_expr(expr);
        }

        let else_jump = self.emit_jump(Opcode::Jump);

        self.patch_jump(then_jump);
        self.emit(Opcode::Pop);

        for expr in &if_else_expr.else_clause.exprs {
            self.compile_expr(expr);
        }

        self.patch_jump(else_jump);
    }

    fn compile_function(&mut self, fun_expr: &FunctionExpr) {
        let current_copy = self.current.clone();
        self.current = CompilerInstance::new(EvalFunctionType::Function);
        *self.current.enclosing_mut() = Box::new(Some(current_copy));

        // Set function name.
        *self.current.function_mut().name_mut() = fun_expr.variable.name.clone();
        *self.current.function_mut().chunk_mut().name_mut() = fun_expr.variable.name.clone();

        self.begin_scope();

        // Compile parameters.
        for p in &fun_expr.declaration.parameters {
            *self.current.function_mut().arity_mut() += 1;
            self.compile_declare_var(p);
        }

        // Compile body.
        self.compile_block(&fun_expr.declaration.body);

        // Create the function object.
        let function = self.end_compiler();

        self.emit(Opcode::Closure);

        let constant_id = self.current_chunk().add_constant(Value::function(function));

        self.emit_byte(constant_id);

        self.compile_define_var(&fun_expr.variable); // TODO fun is always global?
    }

    fn compile_call(&mut self, call: &CallExpr) {
        let arity = call.args.len();
        if arity > 8 {
            panic!() // TODO
        }

        self.compile_expr(&call.callee);

        for arg in &call.args {
            self.compile_expr(arg);
        }

        self.emit(Opcode::Call);
        self.emit_byte(arity as u8);
    }

    fn compile_return(&mut self, return_expr: &ReturnExpr) {
        if *self.current.function_type() == EvalFunctionType::Script {
            panic!("Can't return from top level code.");
        }

        if let Some(expr) = &return_expr.expr {
            self.compile_expr(expr);
            self.emit(Opcode::Return);
        } else {
            self.emit(Opcode::Nil);
            self.emit(Opcode::Return);
        }
    }

    fn compile_for(&mut self, for_expr: &ForExpr) {
        self.begin_scope();

        let variable = Variable::new("x".to_string());
        self.compile_define_var(&variable);

        let loop_start = self.current_chunk().code().len();

        let exit_jump = -1;
        
        self.end_scope();
    }

    fn compile_while(&mut self, while_expr: &WhileExpr) {
        let loop_start = self.current_chunk().code().len();
        self.compile_expr(&while_expr.condition);

        let exit_jump = self.emit_jump(Opcode::JumpIfFalse);
        self.emit(Opcode::Pop);
        self.compile_expr(&while_expr.body);

        self.emit_loop(loop_start);
        self.patch_jump(exit_jump);
        self.emit(Opcode::Pop);
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit(Opcode::Loop);

        let chunk = self.current_chunk();
        let sub = chunk.code().len() - loop_start + 2;

        let lo = ((sub >> 8) & 0xff) as u8;
        let hi = (sub & 0xff) as u8;

        self.emit_byte(lo);
        self.emit_byte(hi);
    }

    fn emit_jump(&mut self, instruction: Opcode) -> usize {
        self.emit(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        return self.current_chunk().code().len() - 2;
    }

    fn patch_jump(&mut self, offset: usize) {
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

    fn resolve_local(&self, name: &String) -> isize {
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
        self.current.locals_mut().push(Local::new(name, -1));
    }

    fn mark_initialized(&mut self) {
        if *self.current.scope_depth() == 0 {
            return;
        }

        let index = &self.current.locals().len() - 1;
        *self.current.locals_mut()[index].depth_mut() = *self.current.scope_depth();
    }

    fn begin_scope(&mut self) {
        *self.current.scope_depth_mut() += 1;
    }

    fn end_scope(&mut self) {
        *self.current.scope_depth_mut() -= 1;

        while self.current.locals().len() > 0
            && self.current.locals()[self.current.locals().len() - 1].depth()
                > self.current.scope_depth()
        {
            self.emit(Opcode::Pop);
            self.current.locals_mut().pop();
        }
    }

    fn end_compiler(&mut self) -> EvalFunction {
        self.emit_return();
        let fun_copy = self.current.function().clone();

        println!("{}", self.current_chunk());

        if let Some(enclosing) = *self.current.enclosing().clone() {
            self.current = enclosing;
        }

        fun_copy
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        self.current.function_mut().chunk_mut()
    }

    fn emit_return(&mut self) {
        self.emit(Opcode::Nil);
        self.emit(Opcode::Return);
    }

    fn emit_string(&mut self, s: &str) {
        self.emit_constant(Value::Obj(s.into()));
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.current_chunk().add_constant(value);
        self.emit(Opcode::Constant);
        self.emit_byte(constant);
    }

    fn emit(&mut self, opcode: Opcode) {
        self.current_chunk().write(opcode, 123); // TODO Line
    }

    fn emit_byte(&mut self, byte: u8) {
        self.current_chunk().write_byte(byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parser::EvalParser;

    fn parse_source(str: &str) -> ModuleAst {
        EvalParser::parse(str).unwrap()
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
