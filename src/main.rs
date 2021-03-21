use crate::parser::parser::EvalParser;
use crate::scanner::lexer::Lexer;

mod scanner;
mod parser;
mod compiler;
mod vm;

fn main() {
    // let tokens = Lexer::parse("5 + 10");
    // println!("{:?}", tokens);

    let input = r#"
        do
            10 + 5
        end
        "#;

    // let input = r#"5 + 10 * 2"#;
    let exprs = EvalParser::parse(input);
    println!("{:?}", exprs);
}
