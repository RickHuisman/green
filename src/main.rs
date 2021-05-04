use crate::vm::VM;

mod compiler;
mod error;
mod repl;
mod syntax;
mod type_system;
mod vm;

fn main() {
    // Repl::run();

    // type_system::repl::repl();

    let source = get_file_contents("/Users/rickhuisman/Documents/rust/green/src/test.txt");
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
