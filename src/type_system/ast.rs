pub struct Type {
    pub node: Box<TypeKind>,
}

pub enum TypeKind {

}

pub trait Infer {
    // TODO Pass Reference
    fn infer() -> Type;
}

pub struct Expr {
    pub node: Box<ExprKind>,
}

impl Infer for Expr {
    fn infer() -> Type {

    }
}

pub enum ExprKind {
    Let(LetExpr),
}

pub struct Variable {
    name: String,
}

pub struct LetExpr {
    variable: Variable,
}