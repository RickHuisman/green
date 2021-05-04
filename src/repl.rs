use crate::vm::VM;
use std::io;
use std::io::BufRead;

pub struct Repl {
    vm: VM,
}

impl Repl {
    fn new() -> Self {
        Repl { vm: VM::new() }
    }

    pub fn run() {
        let mut repl = Repl::new();

        loop {
            match repl.read_line() {
                Ok(line) => repl.eval(&line),
                Err(e) => eprintln!("[error]: {}", e),
            }
        }
    }

    fn eval(&mut self, source: &str) {
        self.vm.interpret(source)
    }

    fn read_line(&self) -> io::Result<String> {
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line)?;
        Ok(line)
    }
}
