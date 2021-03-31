use crate::syntax::token::TokenType;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Expr {
    pub node: Box<ExprKind>,
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "test")
    }
}

impl Expr {
    pub fn new(node: ExprKind) -> Expr {
        Expr { node: Box::new(node) }
    }

    pub fn block(block: BlockExpr) -> Expr {
        Expr::new(ExprKind::Block(block))
    }

    pub fn return_(return_expr: ReturnExpr) -> Expr {
        Expr::new(ExprKind::Return(return_expr))
    }
}

#[derive(PartialEq, Debug)]
pub enum ExprKind {
    Import(ImportExpr),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Block(BlockExpr),
    VarAssign(VarAssignExpr),
    VarSet(VarSetExpr),
    VarGet(VarGetExpr),
    Print(Expr),
    Grouping(GroupingExpr),
    If(IfExpr),
    IfElse(IfElseExpr),
    Function(FunctionExpr),
    Call(CallExpr),
    For(ForExpr),
    Return(ReturnExpr),
}

impl ExprKind {
    pub fn block(self) -> Option<BlockExpr> {
        match self {
            ExprKind::Block(block) => Some(block),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ImportExpr {
    pub module: String,
}

impl ImportExpr {
    pub fn new(module: String) -> Self {
        ImportExpr { module }
    }
}

#[derive(PartialEq, Debug)]
pub enum LiteralExpr {
    Number(f64),
    String(String),
    True,
    False,
    Nil, // TODO Nil???
}

#[derive(PartialEq, Debug)]
pub struct BinaryExpr {
    pub lhs: Expr,
    pub rhs: Expr,
    pub operator: BinaryOperator,
}

impl BinaryExpr {
    pub fn new(lhs: Expr, rhs: Expr, operator: BinaryOperator) -> BinaryExpr {
        BinaryExpr { lhs, rhs, operator }
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
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::Equal,
            TokenType::LessThan => BinaryOperator::LessThan,
            TokenType::LessThanEqual => BinaryOperator::LessThanEqual,
            TokenType::GreaterThan => BinaryOperator::GreaterThan,
            TokenType::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            _ => todo!() // TODO
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct UnaryExpr {
    pub expr: Expr,
    pub operator: UnaryOperator,
}

impl UnaryExpr {
    pub fn new(expr: Expr, operator: UnaryOperator) -> Self {
        UnaryExpr { expr, operator }
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(PartialEq, Debug)]
pub struct BlockExpr {
    pub expressions: Vec<Expr>
}

impl BlockExpr {
    pub fn new(expressions: Vec<Expr>) -> Self {
        BlockExpr { expressions }
    }
}

#[derive(PartialEq, Debug)]
pub struct GroupingExpr {
    pub expr: Expr,
}

impl GroupingExpr {
    pub fn new(expr: Expr) -> Self {
        GroupingExpr { expr }
    }
}

#[derive(PartialEq, Debug)]
pub struct VarAssignExpr {
    pub variable: Variable,
    pub initializer: Expr, // TODO Box???
}

impl VarAssignExpr {
    pub fn new(variable: Variable, initializer: Expr) -> Self {
        VarAssignExpr { variable, initializer }
    }
}

#[derive(PartialEq, Debug)]
pub struct VarSetExpr {
    pub variable: Variable,
    pub initializer: Expr, // TODO Box???
}

impl VarSetExpr {
    pub fn new(variable: Variable, initializer: Expr) -> Self {
        VarSetExpr { variable, initializer }
    }
}

#[derive(PartialEq, Debug)]
pub struct VarGetExpr {
    pub variable: Variable,
}

impl VarGetExpr {
    pub fn new(variable: Variable) -> Self {
        VarGetExpr { variable }
    }
}

#[derive(PartialEq, Debug)]
pub struct Variable {
    pub name: String, // TODO Make &str
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
}

#[derive(PartialEq, Debug)]
pub struct IfExpr {
    pub condition: Expr,
    pub then_clause: Vec<Expr>,
}

impl IfExpr {
    pub fn new(condition: Expr, then_clause: Vec<Expr>) -> Self {
        IfExpr { condition, then_clause }
    }
}

#[derive(PartialEq, Debug)]
pub struct IfElseExpr {
    pub condition: Expr,
    pub then_clause: BlockExpr,
    pub else_clause: BlockExpr,
}

impl IfElseExpr {
    pub fn new(condition: Expr, then_clause: BlockExpr, else_clause: BlockExpr) -> Self {
        IfElseExpr { condition, then_clause, else_clause }
    }
}

#[derive(PartialEq, Debug)]
pub struct FunctionDeclaration {
    pub parameters: Vec<Variable>,
    pub body: BlockExpr,
}

impl FunctionDeclaration {
    pub fn new(parameters: Vec<Variable>, body: BlockExpr) -> Self {
        FunctionDeclaration { parameters, body }
    }
}

#[derive(PartialEq, Debug)]
pub struct FunctionExpr {
    pub variable: Variable,
    pub declaration: FunctionDeclaration,
}

impl FunctionExpr {
    pub fn new(variable: Variable, declaration: FunctionDeclaration) -> Self {
        FunctionExpr { variable, declaration }
    }
}

#[derive(PartialEq, Debug)]
pub struct ForExpr {}

impl ForExpr {
    pub fn new() -> Self {
        ForExpr {}
    }
}

#[derive(PartialEq, Debug)]
pub struct CallExpr {
    pub callee: Expr,
    pub args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(callee: Expr, args: Vec<Expr>) -> Self {
        CallExpr { callee, args }
    }
}

#[derive(PartialEq, Debug)]
pub struct ReturnExpr {
    pub expr: Option<Expr>,
}

impl ReturnExpr {
    pub fn new(expr: Option<Expr>) -> Self {
        ReturnExpr { expr }
    }
}