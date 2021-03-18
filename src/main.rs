use crate::scanner::lexer::Lexer;
use crate::parser::parser::Parser;

mod scanner;
mod parser;

fn main() {
    // let tokens = Lexer::parse("5 + 10");
    // println!("{:?}", tokens);

    let exprs = Parser::parse("5 + 10");
    println!("{:?}", exprs);
}
