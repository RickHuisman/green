use crate::parser::ast::expr::{Expr, ExprKind, LiteralExpr, BinaryExpr, BinaryOperator, UnaryExpr, UnaryOperator};
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::compiler::chunk::Chunk;

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn compile(exprs: Vec<Expr>) -> Chunk {
        let mut compiler = Compiler { chunk: Chunk::new() };

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
            ExprKind::Block(_) => todo!(),
            ExprKind::Print(print) => self.compile_print(print),
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

    fn compile_print(&mut self, expr: Expr) {
        self.compile_expr(expr);
        self.emit(Opcode::Print);
    }

    fn compile_literal(&mut self, literal: LiteralExpr) {
        match literal {
            LiteralExpr::Number(n) => self.emit_constant(Value::Number(n)),
            LiteralExpr::String(s) => todo!(),
            LiteralExpr::True => todo!(),
            LiteralExpr::False => todo!(),
        }
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