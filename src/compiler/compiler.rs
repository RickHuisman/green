use crate::parser::ast::expr::{Expr, ExprKind, LiteralExpr, BinaryExpr, BinaryOperator, UnaryExpr, UnaryOperator, BlockExpr, GroupingExpr, VarSetExpr, VarGetExpr, VarAssignExpr, IfExpr, IfElseExpr};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::compiler::chunk::Chunk;
use crate::compiler::object::Object;
use std::process::exit;
use crate::compiler::local::Local;

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
}

impl Compiler {
    fn new() -> Self {
        let locals = Vec::with_capacity(u8::MAX as usize);
        Compiler {
            chunk: Chunk::new(),
            locals,
            scope_depth: 0,
        }
    }

    pub fn compile(exprs: Vec<Expr>) -> Chunk { // TODO Accept &str not exprs
        let mut compiler = Compiler::new();

        for expr in exprs {
            compiler.compile_expr(expr);
        }

        compiler.chunk
    }

    fn compile_expr(&mut self, expr: Expr) {
        match *expr.node {
            ExprKind::Literal(literal) => self.compile_literal(literal),
            ExprKind::Binary(binary) => self.compile_binary(binary),
            ExprKind::Unary(unary) => self.compile_unary(unary),
            ExprKind::Block(block) => self.compile_block(block),
            ExprKind::Print(print) => self.compile_print(print),
            ExprKind::Grouping(grouping) => self.compile_grouping(grouping),
            ExprKind::VarAssign(var) => self.compile_declare_var(var),
            ExprKind::VarSet(var) => self.compile_set_var(var),
            ExprKind::VarGet(var) => self.compile_get_var(var),
            ExprKind::If(if_expr) => self.compile_if(if_expr),
            ExprKind::IfElse(if_else_expr) => self.compile_if_else(if_else_expr),
        }
    }

    fn compile_binary(&mut self, binary: BinaryExpr) {
        self.compile_expr(*binary.lhs);
        self.compile_expr(*binary.rhs);

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

    fn compile_unary(&mut self, unary: UnaryExpr) {
        self.compile_expr(*unary.expr);

        match unary.operator {
            UnaryOperator::Negate => self.emit(Opcode::Negate),
            UnaryOperator::Not => self.emit(Opcode::Not),
        }
    }

    fn compile_block(&mut self, block: BlockExpr) {
        self.begin_scope();
        for expr in block.expressions {
            self.compile_expr(expr);
        }
        self.end_scope();
    }

    fn compile_print(&mut self, expr: Expr) {
        self.compile_expr(expr);
        self.emit(Opcode::Print);
    }

    fn compile_grouping(&mut self, grouping: GroupingExpr) {
        self.compile_expr(*grouping.expr);
    }

    fn compile_declare_var(&mut self, var: VarAssignExpr) {
        self.compile_expr(var.initializer);

        self.emit(Opcode::DefineGlobal);
        let constant_id = self.chunk.add_constant(Value::Obj(var.variable.name.into()));
        self.emit_byte(constant_id);
    }

    fn compile_set_var(&mut self, var: VarSetExpr) {
        self.compile_expr(var.initializer);

        self.emit(Opcode::SetGlobal);
        let constant_id = self.chunk.add_constant(Value::Obj(var.variable.name.into()));
        self.emit_byte(constant_id);
    }

    fn compile_get_var(&mut self, var: VarGetExpr) {
        self.emit(Opcode::GetGlobal);
        let constant_id = self.chunk.add_constant(Value::Obj(var.variable.name.into()));
        self.emit_byte(constant_id);
    }

    fn compile_if(&mut self, if_expr: IfExpr) {
        self.compile_expr(if_expr.condition);

        // Jump to else clause if false
        let then_jump = self.emit_jump(Opcode::JumpIfFalse);
        self.emit(Opcode::Pop);

        for expr in if_expr.then_clause {
            self.compile_expr(expr);
        }

        let else_jump = self.emit_jump(Opcode::Jump);

        self.patch_jump(then_jump);
        self.emit(Opcode::Pop);

        self.patch_jump(else_jump);
    }

    fn compile_if_else(&mut self, if_else_expr: IfElseExpr) {
        self.compile_expr(if_else_expr.condition);

        // Jump to else clause if false
        let then_jump = self.emit_jump(Opcode::JumpIfFalse);
        self.emit(Opcode::Pop);

        for expr in if_else_expr.then_clause {
            self.compile_expr(expr);
        }

        let else_jump = self.emit_jump(Opcode::Jump);

        self.patch_jump(then_jump);
        self.emit(Opcode::Pop);

        for expr in if_else_expr.else_clause {
            self.compile_expr(expr);
        }

        self.patch_jump(else_jump);
    }

    fn emit_jump(&mut self, instruction: Opcode) -> usize {
        self.emit(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        return self.chunk.code().len() - 2;
    }

    fn patch_jump(&mut self, offset: usize) {
        // -2 to adjust for the bytecode for the jump offset itself.
        let jump = self.chunk.code().len() - offset - 2;

        self.chunk.code_mut()[offset] = ((jump >> 8) & 0xff) as u8;
        self.chunk.code_mut()[offset + 1] = (jump & 0xff) as u8;
    }

    fn compile_literal(&mut self, literal: LiteralExpr) {
        match literal {
            LiteralExpr::Number(n) => self.emit_constant(Value::Number(n)),
            LiteralExpr::String(s) => self.emit_string(&s),
            LiteralExpr::True => todo!(),
            LiteralExpr::False => todo!(),
            _ => todo!() // TODO NilLiteral
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
    }

    fn emit_string(&mut self, s: &str) {
        self.emit_constant(Value::Obj(s.into()));
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        self.emit(Opcode::Constant);
        self.emit_byte(constant);
    }

    fn emit(&mut self, opcode: Opcode) {
        self.chunk.write(opcode, 123); // TODO Line
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte);
    }
}