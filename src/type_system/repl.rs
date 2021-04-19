use std::io;
use std::borrow::BorrowMut;

pub fn repl() {
    loop {
        let line = read_line();
        println!("{:?}", line);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            trim_newline(input.borrow_mut());
            input
        },
        Err(error) => {
            panic!("error: {}", error);
        },
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}