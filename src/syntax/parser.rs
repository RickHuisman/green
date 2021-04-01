use crate::syntax::lexer::Lexer;
use crate::syntax::token::{Token, TokenType, Keyword};
use crate::syntax::rule::{Precedence, get_prefix_rule, get_precedence, get_infix_rule};
use crate::syntax::expr::{Expr, ExprKind, BlockExpr, LiteralExpr, Variable, VarSetExpr, VarGetExpr, VarAssignExpr, IfExpr, IfElseExpr, FunctionDeclaration, FunctionExpr, ReturnExpr, ForExpr, ImportExpr};
use crate::syntax::morpher::morph;
use std::fmt;
use std::fmt::Display;
use crate::syntax::parser::ParserError::UnexpectedToken;

#[derive(Debug)]
pub struct ModuleAst {
    exprs: Vec<Expr>
}

impl ModuleAst {
    pub fn new(exprs: Vec<Expr>) -> Self {
        ModuleAst { exprs }
    }

    pub fn exprs(&self) -> &Vec<Expr> {
        &self.exprs
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(TokenType),
    Expect(TokenType, TokenType),
    UnexpectedEOF,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(unexpected) => write!(f, "Unexpected token {:?}", unexpected),
            ParserError::Expect(expected, actual) =>
                write!(f, "Expected {:?}, got {:?}", expected, actual),
            ParserError::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct EvalParser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> EvalParser<'a> {
    fn new(source: &'a str) -> Self {
        let mut tokens = Lexer::parse(source).unwrap();
        tokens = morph(tokens);
        tokens.reverse();

        EvalParser { tokens }
    }

    pub fn parse(source: &str) -> Result<ModuleAst> {
        let mut parser = EvalParser::new(source);

        let mut exprs = vec![];
        while !parser.match_(TokenType::EOF)? {
            // Consume lines till a non line token is found
            while parser.match_(TokenType::Line)? {
                parser.consume()?;
            }

            exprs.push(parser.parse_top_level_expression()?);

            if !parser.match_(TokenType::EOF)? {
                parser.expect(TokenType::Line)?;
            }
        }

        Ok(ModuleAst::new(exprs))
    }

    // Eval doesn't have statements but "top-level" expressions.
    fn parse_top_level_expression(&mut self) -> Result<Expr> {
        match self.peek_type()? {
            TokenType::Keyword(Keyword::Import) => self.parse_import(),
            TokenType::Keyword(Keyword::Print) => self.parse_print(),
            TokenType::Keyword(Keyword::Def) => self.declare_def(),
            TokenType::Keyword(Keyword::Var) => self.declare_var(),
            TokenType::Keyword(Keyword::Do) => self.parse_do(),
            TokenType::Keyword(Keyword::If) => self.parse_if(),
            TokenType::Keyword(Keyword::For) => self.parse_for(),
            TokenType::Keyword(Keyword::Return) => self.parse_return(),
            _ => self.parse_expression()
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

        // Consume tokens till end of line, this is the path of the module.
        let mut module_path = String::new();
        while !self.match_(TokenType::Line)? {
            module_path.push_str(self.consume()?.source);
        }

        let import_expr = ImportExpr::new(module_path.to_string());
        Ok(Expr::new(ExprKind::Import(import_expr)))
    }

    fn parse_print(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Print))?;
        Ok(Expr::new(ExprKind::Print(self.parse_expression()?)))
    }

    fn declare_def(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Def));

        let identifier = self.expect(TokenType::Identifier)?;

        self.expect(TokenType::LeftParen)?;

        let mut parameters = vec![];
        while !self.match_(TokenType::RightParen)? &&
            !self.match_(TokenType::EOF)? {
            let param = self.expect(TokenType::Identifier)?;
            parameters.push(Variable::new(param.source.to_string()));

            if self.match_(TokenType::Comma)? {
                self.consume()?;
            } else {
                break;
            }
        }

        self.expect(TokenType::RightParen)?;
        self.expect(TokenType::Line)?;

        let body = self.parse_block()?.node.block().unwrap(); // TODO Unwrap

        let fun_decl = FunctionDeclaration::new(
            parameters, body,
        );

        Ok(Expr::new(ExprKind::Function(
            FunctionExpr::new(
                Variable::new(identifier.source.to_string()),
                fun_decl,
            )
        )))
    }

    fn declare_var(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Var));

        let identifier = self.expect(TokenType::Identifier);
        let var = Variable::new(identifier?.source.to_string());

        let mut initializer = Expr::new(ExprKind::Literal(LiteralExpr::Nil));

        // Var has initializer
        if self.match_(TokenType::Equal)? {
            // Pop '=' operator
            self.consume()?;

            initializer = self.parse_expression()?;
        }

        Ok(Expr::new(ExprKind::VarAssign(VarAssignExpr::new(var, initializer))))
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

    fn parse_do(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Do))?;
        self.expect(TokenType::Line)?;

        Ok(self.parse_block()?)
    }

    fn parse_if(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::If));

        let cond = self.parse_expression()?;

        self.expect(TokenType::Keyword(Keyword::Then))?;
        self.expect(TokenType::Line)?;

        let mut then = vec![];
        while !self.match_(TokenType::Keyword(Keyword::End))? &&
            !self.match_(TokenType::Keyword(Keyword::Else))? {
            then.push(self.parse_top_level_expression()?);
            self.expect(TokenType::Line)?;
        }

        let expr_kind = if self.match_(TokenType::Keyword(Keyword::Else))? {
            self.consume()?;

            self.expect(TokenType::Line)?;

            let else_clause = self.parse_block()?.node.block().unwrap(); // TODO Unwrap

            ExprKind::IfElse(IfElseExpr::new(cond, BlockExpr::new(then), else_clause))
        } else {
            self.expect(TokenType::Keyword(Keyword::End));
            ExprKind::If(IfExpr::new(cond, then))
        };

        Ok(Expr::new(expr_kind))
    }

    fn parse_for(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::For))?;

        Ok(Expr::new(ExprKind::For(ForExpr::new())))
    }

    fn parse_return(&mut self) -> Result<Expr> {
        self.expect(TokenType::Keyword(Keyword::Return))?;

        let return_expr = if !self.match_(TokenType::Line)? {
            None
        } else {
            Some(self.parse_top_level_expression()?)
        };

        Ok(Expr::return_(ReturnExpr::new(return_expr)))
    }

    fn parse_block(&mut self) -> Result<Expr> {
        // TODO Check for single line expr: do print(10) end

        let mut exprs = vec![];
        while !self.match_(TokenType::Keyword(Keyword::End))? {
            exprs.push(self.parse_top_level_expression()?);
            self.expect(TokenType::Line)?;
        }

        self.expect(TokenType::Keyword(Keyword::End))?;

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

    #[test]
    fn it_works() {
        let input = r#"
        do
            print(10)
            print(10)
        end
"#;

        let exprs = EvalParser::parse(input);
        println!("{:?}", exprs);
    }

    #[test]
    fn parse_var() {
        let input = r#"
        var x = 10
        x = 10
        print(x)
        "#;

        let exprs = EvalParser::parse(input);
        println!("{:?}", exprs);
    }

    #[test]
    fn parse_if_else() {
        let input = r#"
        if 10 > 5 then
            print(5)
        else
            print(10)
        end
        "#;

        let exprs = EvalParser::parse(input);
        println!("{:?}", exprs);
    }

    #[test]
    fn parse_import() {
        let input = r#"
        import foo.bar
        import util
        import ..bar.foo
        "#;

        let exprs = EvalParser::parse(input);
        println!("{:?}", exprs);
    }
}