use crate::scanner::lexer::Lexer;
use crate::parser::parser::EvalParser;

mod scanner;
mod parser;

fn main() {
    // let tokens = Lexer::parse("5 + 10");
    // println!("{:?}", tokens);

    let exprs = EvalParser::parse("5 + 10 * 2 - 2");
    println!("{:?}", exprs);
}
