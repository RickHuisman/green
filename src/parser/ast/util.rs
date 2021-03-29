// pub fn print_ast(exprs: Vec<Expr>, prefix: &str, children_prefix: &str) {
//     let mut builder = String::new();
//     for expr in exprs {
//         let str_expr = print_expr(expr, &mut builder);
//         builder.push_str(&str_expr);
//     }
//
//     println!("{}", builder);
// }
//
// fn print_expr(expr: Expr, builder: &mut String) -> String {
//     match *expr.node {
//         ExprKind::Literal(literal) => {
//             let str = "[literal: ";
//             builder.push_str(&str);
//             builder.to_string()
//         },
//         ExprKind::Binary(binary) => todo!(),
//         // ExprKind::Block(bloc) => {}
//         ExprKind::Print(print) => {
//             let str = "[print: ".to_owned() + print_expr(print, builder).as_str() + "]";
//             builder.push_str(&str);
//             builder.to_string()
//         },
//         _ => todo!()
//     }
// }