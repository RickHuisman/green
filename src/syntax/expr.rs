use crate::compiler::compiler::Compiler;
use crate::compiler::instance::CompilerInstance;
use crate::compiler::module_resolver::get_module_ast;
use crate::compiler::object::GreenFunctionType;
use crate::compiler::opcode::Opcode;
use crate::compiler::value::Value;
use crate::syntax::token::TokenType;

pub trait Compile {
    fn compile(&self, compiler: &mut Compiler);
}

#[derive(PartialEq, Debug)]
pub struct Expr {
    pub node: Box<ExprKind>,
}

impl Expr {
    pub fn new(node: ExprKind) -> Expr {
        Expr {
            node: Box::new(node),
        }
    }

    pub fn sequence(seq_expr: SequenceExpr) -> Expr {
        Expr::new(ExprKind::Sequence(seq_expr))
    }

    pub fn import(import_expr: ImportExpr) -> Expr {
        Expr::new(ExprKind::Import(import_expr))
    }

    pub fn literal(literal_expr: LiteralExpr) -> Expr {
        Expr::new(ExprKind::Literal(literal_expr))
    }

    pub fn binary(binary: BinaryExpr) -> Expr {
        Expr::new(ExprKind::Binary(binary))
    }

    pub fn block(block: BlockExpr) -> Expr {
        Expr::new(ExprKind::Block(block))
    }

    pub fn var_assign(assign: VarAssignExpr) -> Expr {
        Expr::new(ExprKind::VarAssign(assign))
    }

    pub fn var_set(set: VarSetExpr) -> Expr {
        Expr::new(ExprKind::VarSet(set))
    }

    pub fn var_get(get: VarGetExpr) -> Expr {
        Expr::new(ExprKind::VarGet(get))
    }

    pub fn print(print: PrintExpr) -> Expr {
        Expr::new(ExprKind::Print(print))
    }

    pub fn grouping(group: GroupingExpr) -> Expr {
        Expr::new(ExprKind::Grouping(group))
    }

    pub fn if_else(if_else: IfElseExpr) -> Expr {
        Expr::new(ExprKind::IfElse(if_else))
    }

    pub fn while_(while_expr: WhileExpr) -> Expr {
        Expr::new(ExprKind::While(while_expr))
    }

    pub fn return_(return_expr: ReturnExpr) -> Expr {
        Expr::new(ExprKind::Return(return_expr))
    }

    pub fn get_property(get_property: GetExpr) -> Self {
        Expr::new(ExprKind::GetProperty(get_property))
    }

    pub fn set_property(set_property: SetExpr) -> Self {
        Expr::new(ExprKind::SetProperty(set_property))
    }

    pub fn nil() -> Expr {
        Expr::new(ExprKind::Literal(LiteralExpr::Nil))
    }

    pub fn class(class_expr: ClassExpr) -> Expr {
        Expr::new(ExprKind::Class(class_expr))
    }
}

#[derive(PartialEq, Debug)]
pub enum ExprKind {
    Sequence(SequenceExpr),
    Import(ImportExpr),
    Literal(LiteralExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Block(BlockExpr),
    VarAssign(VarAssignExpr),
    VarSet(VarSetExpr),
    VarGet(VarGetExpr),
    Print(PrintExpr),
    Grouping(GroupingExpr),
    If(IfExpr),
    IfElse(IfElseExpr),
    Function(FunctionExpr),
    Class(ClassExpr),
    Call(CallExpr),
    While(WhileExpr),
    Return(ReturnExpr),
    GetProperty(GetExpr),
    SetProperty(SetExpr),
    Array(ArrayExpr),
    Subscript(SubscriptExpr),
}

impl Compile for ExprKind {
    fn compile(&self, compiler: &mut Compiler) {
        match self {
            ExprKind::Sequence(s) => s.compile(compiler),
            ExprKind::Import(i) => i.compile(compiler),
            ExprKind::Literal(l) => l.compile(compiler),
            ExprKind::Binary(b) => b.compile(compiler),
            ExprKind::Unary(u) => u.compile(compiler),
            ExprKind::Block(b) => b.compile(compiler),
            ExprKind::VarAssign(v) => v.compile(compiler),
            ExprKind::VarSet(v) => v.compile(compiler),
            ExprKind::VarGet(v) => v.compile(compiler),
            ExprKind::Print(p) => p.compile(compiler),
            ExprKind::Grouping(g) => g.compile(compiler),
            ExprKind::If(i) => i.compile(compiler),
            ExprKind::IfElse(e) => e.compile(compiler),
            ExprKind::Function(f) => f.compile(compiler),
            ExprKind::Call(c) => c.compile(compiler),
            ExprKind::While(w) => w.compile(compiler),
            ExprKind::Return(r) => r.compile(compiler),
            ExprKind::Array(a) => a.compile(compiler),
            ExprKind::Subscript(s) => s.compile(compiler),
            ExprKind::Class(c) => c.compile(compiler),
            ExprKind::GetProperty(g) => g.compile(compiler),
            ExprKind::SetProperty(s) => s.compile(compiler),
        }
    }
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
pub struct SequenceExpr {
    pub exprs: Vec<Expr>,
}

impl SequenceExpr {
    pub fn new(exprs: Vec<Expr>) -> Self {
        SequenceExpr { exprs }
    }
}

impl Compile for SequenceExpr {
    fn compile(&self, compiler: &mut Compiler) {
        for expr in &self.exprs {
            compiler.compile_expr(expr);
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

impl Compile for ImportExpr {
    fn compile(&self, compiler: &mut Compiler) {
        let module = get_module_ast(&self.module).unwrap();

        // TODO Only compile top level expressions
        for expr in module.exprs() {
            compiler.compile_expr(expr);
        }
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

impl Compile for LiteralExpr {
    fn compile(&self, compiler: &mut Compiler) {
        match self {
            LiteralExpr::Number(n) => compiler.emit_constant(Value::Number(*n)),
            LiteralExpr::String(s) => compiler.emit_string(&s),
            LiteralExpr::True => compiler.emit_constant(Value::True),
            LiteralExpr::False => compiler.emit_constant(Value::False),
            _ => todo!(), // TODO NilLiteral
        }
    }
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

impl Compile for BinaryExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.lhs);
        compiler.compile_expr(&self.rhs);

        match self.operator {
            BinaryOperator::Add => compiler.emit(Opcode::Add),
            BinaryOperator::Subtract => compiler.emit(Opcode::Subtract),
            BinaryOperator::Multiply => compiler.emit(Opcode::Multiply),
            BinaryOperator::Divide => compiler.emit(Opcode::Divide),
            BinaryOperator::Equal => compiler.emit(Opcode::Equal),
            BinaryOperator::BangEqual => {
                compiler.emit(Opcode::Equal);
                compiler.emit(Opcode::Not);
            }
            BinaryOperator::GreaterThan => compiler.emit(Opcode::Greater),
            BinaryOperator::GreaterThanEqual => {
                compiler.emit(Opcode::Less);
                compiler.emit(Opcode::Not);
            }
            BinaryOperator::LessThan => compiler.emit(Opcode::Less),
            BinaryOperator::LessThanEqual => {
                compiler.emit(Opcode::Greater);
                compiler.emit(Opcode::Not);
            }
        }
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
    pub fn from_token(token_type: TokenType) -> Option<BinaryOperator> {
        let op = match token_type {
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
            _ => return None
        };

        Some(op)
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

impl Compile for UnaryExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.expr);
        compiler.emit(Opcode::from(self.operator.clone()));
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(PartialEq, Debug)]
pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

impl BlockExpr {
    pub fn new(exprs: Vec<Expr>) -> Self {
        BlockExpr { exprs }
    }
}

impl Compile for BlockExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.begin_scope();
        for expr in &self.exprs {
            compiler.compile_expr(expr);
        }
        compiler.end_scope();
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

impl Compile for GroupingExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.expr);
    }
}

#[derive(PartialEq, Debug)]
pub struct VarAssignExpr {
    pub variable: Variable,
    pub initializer: Expr,
}

impl VarAssignExpr {
    pub fn new(variable: Variable, initializer: Expr) -> Self {
        VarAssignExpr {
            variable,
            initializer,
        }
    }
}

impl Compile for VarAssignExpr {
    fn compile(&self, compiler: &mut Compiler) {
        // TODO Check if initialized -> if not init with nil
        compiler.compile_expr(&self.initializer);

        if *compiler.current.scope_depth() > 0 as isize {
            // Local
            compiler.compile_declare_var(&self.variable);
        } else {
            // Global
            compiler.compile_define_var(&self.variable);
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct VarSetExpr {
    pub variable: Variable,
    pub initializer: Expr,
}

impl VarSetExpr {
    pub fn new(variable: Variable, initializer: Expr) -> Self {
        VarSetExpr {
            variable,
            initializer,
        }
    }
}

impl Compile for VarSetExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.initializer);

        let var_name = &self.variable.name;
        let arg = compiler.resolve_local(var_name);
        if arg != -1 {
            // Local
            compiler.emit(Opcode::SetLocal);
            compiler.emit_byte(arg as u8);
        } else {
            // Global
            compiler.emit(Opcode::SetGlobal);
            let str_obj = Value::string(var_name.clone());
            let constant_id = compiler.current_chunk().add_constant(str_obj);
            compiler.emit_byte(constant_id);
        }
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

impl Compile for VarGetExpr {
    fn compile(&self, compiler: &mut Compiler) {
        let var_name = &self.variable.name;
        let arg = compiler.resolve_local(var_name);
        if arg != -1 {
            // Local
            compiler.emit(Opcode::GetLocal);
            compiler.emit_byte(arg as u8);
        } else {
            // Global
            compiler.emit(Opcode::GetGlobal);
            let str_obj = Value::string(var_name.clone());
            let constant_id = compiler.current_chunk().add_constant(str_obj);
            compiler.emit_byte(constant_id);
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct PrintExpr {
    pub expr: Expr,
}

impl PrintExpr {
    pub fn new(expr: Expr) -> PrintExpr {
        PrintExpr { expr }
    }
}

impl Compile for PrintExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.expr);
        compiler.emit(Opcode::Print);
    }
}

#[derive(PartialEq, Debug)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
}

#[derive(PartialEq, Debug)]
pub struct IfExpr {
    pub condition: Expr,
    pub then_clause: Expr,
}

impl IfExpr {
    pub fn new(condition: Expr, then_clause: Expr) -> Self {
        IfExpr {
            condition,
            then_clause,
        }
    }
}

impl Compile for IfExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.condition);

        // Jump to else clause if false
        let then_jump = compiler.emit_jump(Opcode::JumpIfFalse);
        compiler.emit(Opcode::Pop);

        compiler.compile_expr(&self.then_clause);

        let else_jump = compiler.emit_jump(Opcode::Jump);

        compiler.patch_jump(then_jump);
        compiler.emit(Opcode::Pop);

        compiler.patch_jump(else_jump);
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
        IfElseExpr {
            condition,
            then_clause,
            else_clause,
        }
    }
}

impl Compile for IfElseExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.condition);

        // Jump to else clause if false
        let then_jump = compiler.emit_jump(Opcode::JumpIfFalse);
        compiler.emit(Opcode::Pop);

        for expr in &self.then_clause.exprs {
            compiler.compile_expr(expr);
        }

        let else_jump = compiler.emit_jump(Opcode::Jump);

        compiler.patch_jump(then_jump);
        compiler.emit(Opcode::Pop);

        for expr in &self.else_clause.exprs {
            compiler.compile_expr(expr);
        }

        compiler.patch_jump(else_jump);
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
        FunctionExpr {
            variable,
            declaration,
        }
    }
}

impl Compile for FunctionExpr {
    fn compile(&self, compiler: &mut Compiler) {
        let current_copy = compiler.current.clone();
        compiler.current = CompilerInstance::new(GreenFunctionType::Function);
        *compiler.current.enclosing_mut() = Box::new(Some(current_copy));

        // Set function name.
        *compiler.current.function_mut().name_mut() = self.variable.name.clone();
        *compiler.current.function_mut().chunk_mut().name_mut() = Some(self.variable.name.clone());

        compiler.begin_scope();

        // Compile parameters.
        for p in &self.declaration.parameters {
            *compiler.current.function_mut().arity_mut() += 1;
            compiler.compile_declare_var(p);
        }

        // Compile body.
        self.declaration.body.compile(compiler); // TODO works???
                                                 // compiler.compile_expr(&self.declaration.body);

        // Create the function object.
        let function = compiler.end_compiler();

        compiler.emit(Opcode::Closure);

        let constant_id = compiler
            .current_chunk()
            .add_constant(Value::function(function));

        compiler.emit_byte(constant_id);

        compiler.compile_define_var(&self.variable); // TODO fun is always global?
    }
}

#[derive(PartialEq, Debug)]
pub struct ClassExpr {
    pub name: Variable,
}

impl ClassExpr {
    pub fn new(name: Variable) -> Self {
        ClassExpr { name }
    }
}

impl Compile for ClassExpr {
    fn compile(&self, compiler: &mut Compiler) {
        let name_constant = compiler
            .current_chunk()
            .add_constant(Value::string(self.name.name.to_string()));
        compiler.compile_declare_var(&self.name);

        compiler.emit(Opcode::Class);
        compiler.emit_byte(name_constant);
        compiler.compile_define_var(&self.name);
    }
}

#[derive(PartialEq, Debug)]
pub struct WhileExpr {
    pub condition: Expr,
    pub body: Expr,
}

impl WhileExpr {
    pub fn new(condition: Expr, body: Expr) -> Self {
        WhileExpr { condition, body }
    }
}

impl Compile for WhileExpr {
    fn compile(&self, compiler: &mut Compiler) {
        let loop_start = compiler.current_chunk().code().len();
        compiler.compile_expr(&self.condition);

        let exit_jump = compiler.emit_jump(Opcode::JumpIfFalse);
        compiler.emit(Opcode::Pop);
        compiler.compile_expr(&self.body);

        compiler.emit_loop(loop_start);
        compiler.patch_jump(exit_jump);
        compiler.emit(Opcode::Pop);
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

impl Compile for CallExpr {
    fn compile(&self, compiler: &mut Compiler) {
        let arity = self.args.len();
        if arity > 8 {
            panic!() // TODO
        }

        compiler.compile_expr(&self.callee);

        for arg in &self.args {
            compiler.compile_expr(arg);
        }

        compiler.emit(Opcode::Call);
        compiler.emit_byte(arity as u8);
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

impl Compile for ReturnExpr {
    fn compile(&self, compiler: &mut Compiler) {
        if *compiler.current.function_type() == GreenFunctionType::Script {
            panic!("Can't return from top level code."); // TODO Turn into error
        }

        if let Some(expr) = &self.expr {
            compiler.compile_expr(expr);
            compiler.emit(Opcode::Return);
        } else {
            compiler.emit_return()
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ArrayExpr {
    pub exprs: Option<Vec<Expr>>,
}

impl ArrayExpr {
    pub fn new(exprs: Option<Vec<Expr>>) -> Self {
        ArrayExpr { exprs }
    }
}

impl Compile for ArrayExpr {
    fn compile(&self, compiler: &mut Compiler) {
        if let Some(exprs) = &self.exprs {
            for expr in exprs {
                expr.node.compile(compiler);
            }
        }

        compiler.emit(Opcode::NewArray);
        let exprs_len = self.exprs
            .as_ref()
            .map_or_else(|| 0, |a| a.len());
        compiler.emit_byte(exprs_len as u8);
    }
}

#[derive(PartialEq, Debug)]
pub struct SubscriptExpr {
    callee: Expr, // TODO Naming???
    index: Expr,
    expr: Option<Expr> // TODO Comment
}

impl SubscriptExpr {
    pub fn new(callee: Expr, index: Expr,  expr: Option<Expr>) -> SubscriptExpr {
        SubscriptExpr { callee, index, expr }
    }
}

impl Compile for SubscriptExpr {
    fn compile(&self, compiler: &mut Compiler) {
        self.callee.node.compile(compiler);
        self.index.node.compile(compiler);

        if let Some(expr) = &self.expr {
            expr.node.compile(compiler);
            compiler.emit(Opcode::StoreSubscript);
        } else {
            compiler.emit(Opcode::IndexSubscript);
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct GetExpr {
    expr: Expr, // TODO Rename
    property: String,
}

impl GetExpr {
    pub fn new(expr: Expr, property: String) -> Self {
        GetExpr { expr, property }
    }
}

impl Compile for GetExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.expr);

        compiler.emit(Opcode::GetProperty);

        let property_constant = compiler
            .current_chunk()
            .add_constant(Value::string(self.property.to_string()));
        compiler.emit_byte(property_constant);
    }
}

#[derive(PartialEq, Debug)]
pub struct SetExpr {
    lhs: Expr,
    rhs: Expr,
    property: String,
}

impl SetExpr {
    pub fn new(lhs: Expr, rhs: Expr, property: String) -> Self {
        SetExpr { lhs, rhs, property }
    }
}

impl Compile for SetExpr {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_expr(&self.lhs);
        compiler.compile_expr(&self.rhs);

        compiler.emit(Opcode::SetProperty);

        let property_constant = compiler
            .current_chunk()
            .add_constant(Value::string(self.property.to_string()));
        compiler.emit_byte(property_constant);
    }
}