use crate::scanner::token::{Token, TokenType};

#[derive(PartialEq, Debug)]
pub struct Expr {
    pub node: ExprKind,
}

impl Expr {
    pub fn new(node: ExprKind) -> Expr {
        Expr { node }
    }
}

#[derive(PartialEq, Debug)]
pub enum ExprKind {
    Literal(Literal),
    Binary(BinaryExpr),
    //Var(Variable, Expr)
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Number(f64),
    String(String), // TODO Make &'a str
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub operator: BinaryOperator,
}

impl BinaryExpr {
    pub fn new(lhs: Expr, rhs: Expr, operator: BinaryOperator) -> BinaryExpr {
        BinaryExpr { lhs: Box::new(lhs), rhs: Box::new(rhs), operator }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BinaryOperator {
    Equal,
    BangEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Subtract,
    Add,
    Divide,
    Multiply,
}

impl BinaryOperator {
    pub fn from_token(token_type: TokenType) -> BinaryOperator {
        match token_type {
            TokenType::Minus => BinaryOperator::Subtract,
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Star => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            // TokenType::Bang => BinaryOperator::Ba TODO ???
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::Equal,
            TokenType::LessThan => BinaryOperator::LessThan,
            TokenType::LessThanEqual => BinaryOperator::LessThanEqual,
            TokenType::GreaterThan => BinaryOperator::GreaterThan,
            TokenType::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            _ => panic!("TODO") // TODO
        }
    }
}

// struct Variable {
//
// }