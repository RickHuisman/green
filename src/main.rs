use crate::vm::vm::VM;
use std::env;
use std::process::exit;

mod compiler;
mod syntax;
mod vm;

fn main() {
    if env::args().len() == 0 {
        exit(1);
    }

    let mut args = env::args();
    args.next(); // Pop app path

    for arg in args {
        let source = get_file_contents(&arg);
        run(&source.unwrap());
    }
}

fn run(source: &str) {
    let mut vm = VM::new();
    vm.interpret(source);
}

fn get_file_contents(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}