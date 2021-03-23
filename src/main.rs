use crate::parser::parser::EvalParser;
use crate::scanner::lexer::Lexer;
use crate::compiler::chunk::Chunk;
use crate::vm::vm::VM;
use crate::compiler::opcode::Opcode;
use crate::parser::ast::expr::{Expr, ExprKind, LiteralExpr, BinaryExpr, BinaryOperator, UnaryExpr, UnaryOperator};
use crate::compiler::compiler::Compiler;

mod scanner;
mod parser;
mod compiler;
mod vm;

fn main() {
    let source = get_file_contents(
        "/Users/rickhuisman/Documents/rust/eval/src/test.txt"
    );
    run(&source.unwrap());
}

fn run(source: &str) {
    let exprs = EvalParser::parse(source);

    let chunk = Compiler::compile(exprs);
    println!("{}", chunk);
    let mut vm = VM::new();
    vm.interpret(&chunk);
}

fn get_file_contents(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}