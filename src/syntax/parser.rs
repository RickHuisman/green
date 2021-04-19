use crate::error::ParserError;
use crate::syntax::expr::ExprKind::{Binary, Literal};
use crate::syntax::expr::{
    BinaryExpr, BinaryOperator, BlockExpr, Expr, ExprKind, FunctionDeclaration, FunctionExpr,
    IfElseExpr, IfExpr, ImportExpr, LiteralExpr, PrintExpr, ReturnExpr, SequenceExpr,
    VarAssignExpr, VarGetExpr, VarSetExpr, Variable, WhileExpr,
};
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::ParserError::UnexpectedToken;
use crate::syntax::rule::{get_infix_rule, get_precedence, get_prefix_rule, Precedence};
use crate::syntax::token::{Keyword, Token, TokenType};
use std::fmt;
use std::fmt::Display;
use crate::syntax::token::TokenType::Semicolon;

#[derive(Debug, PartialEq)]
pub struct ModuleAst {
    exprs: Vec<Expr>,
}

impl ModuleAst {
    pub fn new(exprs: Vec<Expr>) -> Self {
        ModuleAst { exprs }
    }

    pub fn exprs(&self) -> &Vec<Expr> {
        &self.exprs
    }
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct GreenParser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> GreenParser<'a> {
    fn new(source: &'a str) -> Self {
        let mut tokens = Lexer::parse(source).unwrap();
        tokens.reverse();

        GreenParser { tokens }
    }

    pub fn parse(source: &str) -> Result<ModuleAst> {
        let mut parser = GreenParser::new(source);

        let mut exprs = vec![];
        while !parser.match_(TokenType::EOF)? {
            exprs.push(parser.parse_top_level_expression()?);
        }

        Ok(ModuleAst::new(exprs))
    }

    // Green doesn't have statements but "top-level" expressions.
    fn parse_top_level_expression(&mut self) -> Result<Expr> {
        match self.peek_type()? {
            TokenType::Keyword(Keyword::Import) => self.parse_import(),
            TokenType::Keyword(Keyword::Print) => self.parse_print(),
            TokenType::Keyword(Keyword::Fun) => self.declare_def(),
            TokenType::Keyword(Keyword::Var) => self.declare_var(true),
            TokenType::Keyword(Keyword::Val) => self.declare_var(false),
            TokenType::LeftBrace => self.parse_block(),
            TokenType::Keyword(Keyword::If) => self.parse_if(),
            TokenType::Keyword(Keyword::While) => self.parse_while(),
            TokenType::Keyword(Keyword::For) => self.parse_for(),
            TokenType::Keyword(Keyword::Return) => self.parse_return(),
            _ => {
                let expr = self.parse_expression();
                self.expect(TokenType::Semicolon)?;
                expr
            },
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_precedence(Precedence::None)
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expr> {
        // Prefix
        let token = self.consume()?;

        if let Some(prefix) = get_prefix_rule(&token.token_type) {
            let left = prefix.parse(self, token)?;

            // Infix
            if !self.is_empty() {
                Ok(self.parse_infix(left, precedence as u8)?)
            } else {
                Ok(left)
            }
        } else {
            Err(UnexpectedToken(token.token_type))
        }
    }

    fn parse_infix(&mut self, left: Expr, precedence: u8) -> Result<Expr> {
        let mut infix = left;

        loop {
            if self.is_empty() {
                break;
            }

            let current_precedence = get_precedence(self.peek()?);
            if precedence >= current_precedence as u8 {
                break;
            }

            let token = self.consume()?;
            if let Some(rule) = get_infix_rule(&token.token_type) {
                infix = rule.parse(self, infix, token)?;
            }
        }

        Ok(infix)
    }

    fn parse_import(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Import))?;

        // Consume tokens till end of line; this is the path of the module.
        let mut module_path = String::new();
        // while !self.match_(TokenType::Line)? {
        //     module_path.push_str(self.consume()?.source);
        // }

        let import_expr = ImportExpr::new(module_path.to_string());
        Ok(Expr::import(import_expr))
    }

    fn parse_print(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Print))?;
        let expr = self.parse_expression()?;
        self.expect(TokenType::Semicolon);
        Ok(Expr::print(PrintExpr::new(expr)))
    }

    fn declare_def(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Fun));

        let identifier = self.expect(TokenType::Identifier)?;

        self.expect(TokenType::LeftParen)?;

        let mut parameters = vec![];
        while !self.match_(TokenType::RightParen)?
            && !self.match_(TokenType::EOF)? {
            let param = self.expect(TokenType::Identifier)?;
            self.expect(TokenType::Colon)?;
            let param_type = self.expect(TokenType::Identifier)?;

            // TODO Pass type_system
            parameters.push(Variable::new(param.source.to_string()));

            if self.match_(TokenType::Comma)? {
                self.consume()?;
            } else {
                break;
            }
        }

        self.expect(TokenType::RightParen)?;

        // TODO Check if fun has return type_system
        self.expect(TokenType::Arrow);

        let return_type = self.expect(TokenType::Identifier)?;

        let body = self.parse_block()?.node.block().unwrap(); // TODO Unwrap

        let fun_decl = FunctionDeclaration::new(parameters, body);

        Ok(Expr::new(ExprKind::Function(FunctionExpr::new(
            Variable::new(identifier.source.to_string()),
            fun_decl,
        ))))
    }

    fn declare_var(&mut self, mutable: bool) -> Result<Expr> {
        self.consume()?; // Consume "var"

        let identifier = self.expect(TokenType::Identifier)?;
        let var = Variable::new(identifier.source.to_string());

        let mut initializer = Expr::new(ExprKind::Literal(LiteralExpr::Nil));

        // Var has initializer
        if self.match_(TokenType::Equal)? {
            // Consume '=' operator
            self.consume()?;

            initializer = self.parse_expression()?;
        }

        self.expect(TokenType::Semicolon)?;

        Ok(Expr::var_assign(VarAssignExpr::new(var, initializer)))
    }

    pub fn parse_var(&mut self, identifier: Token) -> Result<Expr> {
        let var = Variable::new(identifier.source.to_string());

        // Var has initializer
        let expr_kind = if self.match_(TokenType::Equal)? {
            // Pop '=' operator
            self.consume()?;

            let initializer = self.parse_expression()?;
            ExprKind::VarSet(VarSetExpr::new(var, initializer))
        } else {
            ExprKind::VarGet(VarGetExpr::new(var))
        };

        Ok(Expr::new(expr_kind))
    }

    fn parse_if(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::If));

        let cond = self.parse_expression()?;
        let then = self.parse_block()?.node.block().unwrap();

        let expr_kind = if self.match_(TokenType::Keyword(Keyword::Else))? {
            self.consume()?;

            let else_clause = self.parse_block()?.node.block().unwrap(); // TODO Unwrap

            ExprKind::IfElse(IfElseExpr::new(cond, then, else_clause))
        } else {
            ExprKind::If(IfExpr::new(cond, Expr::sequence(
                SequenceExpr::new(then.exprs))
            ))
        };

        Ok(Expr::new(expr_kind))
    }

    fn parse_while(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::While))?;
        let cond = self.parse_expression()?;

        let body = self.parse_block()?;

        let test = Expr::while_(WhileExpr::new(cond, body));
        println!("{:?}", test);

        Ok(test)
    }

    fn parse_for(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::For))?;

        // Initializer
        let var_ident = self.expect(TokenType::Identifier)?;

        let mut sequence: Vec<Expr> = vec![];

        // Condition
        self.expect(TokenType::Keyword(Keyword::In))?;
        let x_init = self.expect(TokenType::Number)?;

        let op = match self.peek_type()? {
            TokenType::Keyword(Keyword::To) => BinaryOperator::LessThan,
            TokenType::Keyword(Keyword::DownTo) => BinaryOperator::GreaterThan,
            _ => todo!(),
        };
        self.consume()?;

        let max_val = self.expect(TokenType::Number)?;

        let step_incr = if self.match_(TokenType::Keyword(Keyword::Step))? {
            self.consume()?;

            self.expect(TokenType::Number)?
                .source
                .parse::<f64>()
                .unwrap()
        } else {
            1.0
        };

        let var_decl = Expr::new(ExprKind::VarAssign(VarAssignExpr::new(
            Variable::new(var_ident.source.to_string()),
            Expr::new(ExprKind::Literal(LiteralExpr::Number(
                x_init.source.parse::<f64>().unwrap(),
            ))),
        )));
        sequence.push(var_decl);

        let condition = Expr::new(ExprKind::Binary(BinaryExpr::new(
            Expr::new(ExprKind::VarGet(VarGetExpr::new(Variable::new(
                var_ident.source.to_string(),
            )))),
            Expr::new(ExprKind::Literal(LiteralExpr::Number(
                max_val.source.parse::<f64>().unwrap(),
            ))),
            op,
        )));

        // Parse body
        let incr_op = match op {
            BinaryOperator::LessThan => BinaryOperator::Add,
            BinaryOperator::GreaterThan => BinaryOperator::Subtract,
            _ => todo!(),
        };

        let incr_expr = VarSetExpr::new(
            Variable::new(var_ident.source.to_string()),
            Expr::new(ExprKind::Binary(BinaryExpr::new(
                Expr::new(ExprKind::VarGet(VarGetExpr::new(Variable::new(
                    var_ident.source.to_string(),
                )))),
                Expr::new(ExprKind::Literal(LiteralExpr::Number(step_incr))),
                incr_op,
            ))),
        );

        let test_body = self.parse_block()?;
        let mut foo = vec![];
        foo.push(test_body);
        foo.push(Expr::new(ExprKind::VarSet(incr_expr)));

        let body = Expr::new(ExprKind::Sequence(SequenceExpr::new(foo)));

        let while_expr = Expr::while_(WhileExpr::new(condition, body));
        sequence.push(while_expr);

        Ok(Expr::new(ExprKind::Sequence(SequenceExpr::new(sequence))))
    }

    fn parse_return(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Return))?;

        let return_expr = if self.match_(TokenType::Colon)? { // TODO
            None
        } else {
            Some(self.parse_top_level_expression()?)
        };

        Ok(Expr::return_(ReturnExpr::new(return_expr)))
    }

    fn parse_block(&mut self) -> Result<Expr> {
        self.consume()?; // Consume '{'

        let mut exprs = vec![];
        while !self.match_(TokenType::RightBrace)? &&
            !self.match_(TokenType::EOF)?
        {
            exprs.push(self.parse_top_level_expression()?);
        }

        self.expect(TokenType::RightBrace)?;

        Ok(Expr::block(BlockExpr::new(exprs)))
    }

    pub fn match_(&mut self, token_type: TokenType) -> Result<bool> {
        Ok(self.peek_type()? == token_type)
    }

    fn peek_type(&self) -> Result<TokenType> {
        // Ok(self.peek()?.token_type)
        if self.tokens.len() == 0 {
            return Ok(TokenType::EOF);
        }
        Ok(self.peek()?.token_type)
    }

    fn peek(&self) -> Result<&Token<'a>> {
        // TODO unwrap_or_else
        match self.tokens.last() {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    pub fn expect(&mut self, expect: TokenType) -> Result<Token<'a>> {
        if self.match_(expect)? {
            Ok(self.consume()?)
        } else {
            Err(ParserError::Expect(expect, self.peek_type()?))
        }
    }

    pub fn consume(&mut self) -> Result<Token<'a>> {
        // TODO unwrap_or_else
        match self.tokens.pop() {
            Some(token) => Ok(token),
            None => Err(ParserError::UnexpectedEOF),
        }
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::syntax::expr::GroupingExpr;

    #[test]
    fn parse_block() {
        let expected_exprs = Expr::block(BlockExpr::new(
            vec![
                Expr::print(PrintExpr::new(
                    Expr::grouping(GroupingExpr::new(Expr::literal(
                        LiteralExpr::Number(1.0)))
                    ))
                ),
                Expr::print(PrintExpr::new(
                    Expr::grouping(GroupingExpr::new(Expr::literal(
                        LiteralExpr::Number(5.0)))
                    ))
                ),
            ]
        ));
        let expect = ModuleAst::new(vec![expected_exprs]);

        let input = r#"
        {
            print(1);
            print(5);
        }"#;
        let actual = GreenParser::parse(input).unwrap();

        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_declare_var() {
        let expected_exprs = vec![
            Expr::var_assign(VarAssignExpr::new(
                Variable::new("x".to_string()),
                Expr::literal(
                    LiteralExpr::Number(5.0)
                ))
            ),
        ];
        let expect = ModuleAst::new(expected_exprs);

        let input = r#"
        var x = 5;
        "#;

        let actual = GreenParser::parse(input).unwrap();

        assert_eq!(expect, actual)
    }

    #[test]
    fn parse_set_var() {
        let expected_exprs = vec![
            Expr::var_set(VarSetExpr::new(
                Variable::new("x".to_string()),
                Expr::literal(
                    LiteralExpr::Number(5.0)
                ))
            ),
        ];
        let expect = ModuleAst::new(expected_exprs);

        let input = r#"
        x = 5;
        "#;
        let actual = GreenParser::parse(input).unwrap();

        assert_eq!(expect, actual)
    }

    #[test]
    fn parse_get_var() {
        let expected_exprs = vec![
            Expr::var_assign(VarAssignExpr::new(
                Variable::new("x".to_string()),
                Expr::literal(
                    LiteralExpr::Number(5.0)
                ))
            ),
            Expr::var_assign(VarAssignExpr::new(
                Variable::new("y".to_string()),
                Expr::var_get(VarGetExpr::new(
                    Variable::new("x".to_string())
                ))
            )),
        ];
        let expect = ModuleAst::new(expected_exprs);

        let input = r#"
        var x = 5;
        var y = x;
        "#;
        let actual = GreenParser::parse(input).unwrap();

        assert_eq!(expect, actual)
    }

    #[test]
    fn parse_if_else() {
        let empty_vec: Vec<Expr> = vec![]; // FIXME
        let empty_vec2: Vec<Expr> = vec![];
        let expected_exprs = vec![
            Expr::if_else(IfElseExpr::new(
                Expr::binary(BinaryExpr::new(
                    Expr::literal(LiteralExpr::Number(10.0)),
                    Expr::literal(LiteralExpr::Number(5.0)),
                    BinaryOperator::GreaterThan
                )),
                BlockExpr::new(empty_vec),
                BlockExpr::new(empty_vec2),
            ))
        ];
        let expect = ModuleAst::new(expected_exprs);

        let input = r#"
        if 10 > 5 {
        } else {
        }
        "#;
        let actual = GreenParser::parse(input).unwrap();

        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_import() {
        let input = r#"
        import foo.bar
        import util
        import ..bar.foo
        "#;

        let exprs = GreenParser::parse(input);
        println!("{:?}", exprs);
    }

    #[test]
    fn parse_fun() {
        let expected_exprs = vec![
            Expr::new(ExprKind::Function(FunctionExpr::new(
                Variable::new("double".to_string()),
                FunctionDeclaration::new(
                    vec![
                        Variable::new("x".to_string()),
                    ],
                    BlockExpr::new(vec![
                        Expr::return_(ReturnExpr::new(
                            Some(Expr::binary(BinaryExpr::new(
                                Expr::var_get(VarGetExpr::new(
                                    Variable::new("x".to_string())
                                )),
                                Expr::literal(LiteralExpr::Number(2.0)),
                                BinaryOperator::Multiply
                            ))),
                        ))
                    ])
                )
            )))
        ];
        let expect = ModuleAst::new(expected_exprs);

        let input = r#"
        fun double(x: Int) -> Int {
            return x * 2;
        }
        "#;
        let actual = GreenParser::parse(input).unwrap();

        assert_eq!(expect, actual);
    }

    #[test]
    fn parse_while() {
        let input = r#"
        var x = 0;
        while x < 10 {
            x = x + 1;
        }
        "#;

        let exprs = GreenParser::parse(input);
        println!("{:#?}", exprs);
    }
}
