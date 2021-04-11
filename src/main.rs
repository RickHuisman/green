use crate::compiler::compiler::Compiler;
use crate::syntax::expr::{Compile, Expr, ExprKind, LiteralExpr};
use crate::vm::vm::VM;
use std::env;
use std::process::exit;

mod compiler;
mod error;
mod syntax;
mod vm;

fn main() {
    // let literal = Expr::new(ExprKind::Literal(LiteralExpr::Number(10.0)));

    let source = get_file_contents("/Users/rickhuisman/Documents/rust/eval/src/test.txt");
    run(&source.unwrap());

    // if env::args().len() == 0 {
    //     exit(1);
    // }
    //
    // let mut args = env::args();
    // args.next(); // Pop app path
    //
    // for arg in args {
    //     let source = get_file_contents(&arg);
    //     run(&source.unwrap());
    // }
}

fn run(source: &str) {
    let mut vm = VM::new();
    vm.interpret(source);
}

fn get_file_contents(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
