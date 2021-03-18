pub struct Expr {
    pub node: ExprKind,
}

pub enum ExprKind {
    // Binary(Binary)
    Literal(Literal),
    Var(Variable, Expr)
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Number(f64),
    String(String), // TODO Make &'a str
    True,
    False,
}

// #[derive(PartialEq, Debug)]
// pub struct Binary {
//     pub lhs: Box<Expr>,
//     pub rhs: Box<Expr>,
//     pub operator: BinaryOperator,
// }
//
// #[derive(PartialEq, Debug, Copy, Clone)]
// pub enum BinaryOperator {
//     Equal,
//     BangEq,
//     GreaterThan,
//     GreaterThanEq,
//     LessThan,
//     LessThanEq,
//     Minus,
//     Plus,
//     Slash,
//     Star,
// }

struct Variable {

}