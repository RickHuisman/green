use crate::parser::parser::EvalParser;
use crate::scanner::lexer::Lexer;
use crate::compiler::chunk::Chunk;
use crate::compiler::Opcode::Opcode;
use crate::vm::vm::VM;

mod scanner;
mod parser;
mod compiler;
mod vm;

fn main() {
    let mut chunk = Chunk::new();

    let c1 = chunk.add_constant(1.2);
    chunk.write(Opcode::Constant, 123);
    chunk.write_byte(c1);

    chunk.write(Opcode::Return, 123);

    println!("{}", chunk);

    // let tokens = Lexer::parse("5 + 10");
    // println!("{:?}", tokens);

    // let input = r#"
    //     do
    //         10 + 5
    //     end
    //     "#;
    //
    // // let input = r#"5 + 10 * 2"#;
    // let exprs = EvalParser::parse(input);
    // println!("{:?}", exprs);
}