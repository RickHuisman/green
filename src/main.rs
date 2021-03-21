use crate::parser::parser::EvalParser;
use crate::scanner::lexer::Lexer;
use crate::compiler::chunk::Chunk;
use crate::vm::vm::VM;
use crate::compiler::opcode::Opcode;
use crate::parser::ast::expr::{Expr, ExprKind, LiteralExpr, BinaryExpr, BinaryOperator};
use crate::compiler::compiler::Compiler;

mod scanner;
mod parser;
mod compiler;
mod vm;

fn main() {
    // let mut chunk = Chunk::new();
    //
    // let c1 = chunk.add_constant(1.2);
    // chunk.write(Opcode::Constant, 123);
    // chunk.write_byte(c1);
    //
    // chunk.write(Opcode::Return, 123);
    //
    // println!("{}", chunk);

    // let tokens = Lexer::parse("5 + 10");
    // println!("{:?}", tokens);

    // let input = r#"print(10)"#;
    //
    // // let input = r#"5 + 10 * 2"#;
    // let exprs = EvalParser::parse(input);
    // println!("{:?}", exprs);

    let exprs = vec![
        Expr::new(
            ExprKind::Print(
                Expr::new(
                //     ExprKind::Binary(
                //         BinaryExpr::new(
                //             Expr::new(ExprKind::Literal(LiteralExpr::Number(40.0))),
                //             Expr::new(ExprKind::Literal(LiteralExpr::Number(10.0))),
                //             BinaryOperator::Add,
                //         )
                //     )
                    ExprKind::Literal(LiteralExpr::Number(40.0))
                )
            )
        ),
    ];

    let chunk = Compiler::compile(exprs);
    println!("{}", chunk);
    let mut vm = VM::new();
    vm.interpret(&chunk);
}