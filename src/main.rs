use crate::vm::vm::VM;

mod compiler;
mod syntax;
mod vm;

fn main() {
    let source = get_file_contents("/Users/rickhuisman/Documents/rust/eval/src/test.txt");
    run(&source.unwrap());
}

fn run(source: &str) {
    let mut vm = VM::new();
    vm.interpret(source);
}

fn get_file_contents(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
