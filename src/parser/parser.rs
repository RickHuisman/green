use crate::scanner::lexer::Lexer;
use crate::scanner::token::{Token, TokenType, Keyword};
use crate::parser::rule::{Precedence, get_prefix_rule, get_precedence, get_infix_rule};
use crate::parser::ast::expr::{Expr, ExprKind, BlockExpr, LiteralExpr, Variable, VarSetExpr, VarGetExpr, VarAssignExpr, IfExpr, IfElseExpr, FunctionDeclaration, FunctionExpr, ReturnExpr};
use crate::scanner::morpher::morph;

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

    pub fn parse(source: &str) -> Vec<Expr> {
        let mut parser = EvalParser::new(source);

        let mut exprs = vec![];
        while !parser.match_(TokenType::EOF) {
            // Consume lines till a non line token is found
            while parser.match_(TokenType::Line) {
                parser.consume();
            }

            exprs.push(parser.parse_top_level_expression());

            if !parser.match_(TokenType::EOF) {
                parser.expect(TokenType::Line);
            }
        }

        exprs
    }

    // Eval doesn't have statements but "top-level" expressions.
    fn parse_top_level_expression(&mut self) -> Expr {
        match self.peek_type() {
            TokenType::Keyword(Keyword::Print) => self.parse_print(),
            TokenType::Keyword(Keyword::Def) => self.declare_def(),
            TokenType::Keyword(Keyword::Var) => self.declare_var(),
            TokenType::Keyword(Keyword::Do) => self.parse_do(),
            TokenType::Keyword(Keyword::If) => self.parse_if(),
            TokenType::Keyword(Keyword::Return) => self.parse_return(),
            _ => self.parse_expression()
        }
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::None)
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Expr {
        // Prefix
        let token = self.consume();

        if let Some(prefix) = get_prefix_rule(&token.token_type) {
            let left = prefix.parse(self, token);

            // Infix
            if !self.is_empty() {
                self.parse_infix(left, precedence as u8)
            } else {
                left
            }
        } else {
            panic!("Unexpected token {:?}.", token);
        }
    }

    fn parse_infix(&mut self, left: Expr, precedence: u8) -> Expr {
        let mut infix2 = left;

        loop {
            if self.is_empty() {
                break;
            }

            let current_precedence = get_precedence(self.peek());
            if precedence >= current_precedence as u8 {
                break
            }

            let token = self.consume();
            if let Some(infix) = get_infix_rule(&token.token_type) {
                infix2 = infix.parse(self, infix2, token);
            }
        }

        infix2
    }

    fn parse_print(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::Print));
        Expr::new(ExprKind::Print(self.parse_expression()))
    }

    fn declare_def(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::Def));

        let identifier = self.expect(TokenType::Identifier);

        self.expect(TokenType::LeftParen);

        let mut parameters = vec![];
        while !self.match_(TokenType::RightParen) &&
            !self.match_(TokenType::EOF) {
            let param = self.expect(TokenType::Identifier);
            parameters.push(Variable::new(param.source.to_string()));

            if self.match_(TokenType::Comma) {
                self.consume();
            }
            else {
                break;
            }
        }

        self.expect(TokenType::RightParen);

        self.expect(TokenType::Line);

        let body = self.parse_block().node.block().unwrap(); // TODO Unwrap

        let fun_decl = FunctionDeclaration::new(
            parameters, body
        );
        Expr::new(ExprKind::Function(
            FunctionExpr::new(
                Variable::new(identifier.source.to_string()),
                fun_decl,
            )
        ))
    }

    fn declare_var(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::Var));

        let identifier = self.expect(TokenType::Identifier);
        let var = Variable::new(identifier.source.to_string());

        let mut initializer = Expr::new(ExprKind::Literal(LiteralExpr::Nil));

        // Var has initializer
        if self.match_(TokenType::Equal) {
            // Pop '=' operator
            self.consume();

            initializer = self.parse_expression();
        }

        Expr::new(ExprKind::VarAssign(VarAssignExpr::new(var, initializer)))
    }

    pub fn parse_var(&mut self, identifier: Token) -> Expr {
        let var = Variable::new(identifier.source.to_string());

        // Var has initializer
        let expr_kind = if self.match_(TokenType::Equal) {
            // Pop '=' operator
            self.consume();

            let initializer = self.parse_expression();
            ExprKind::VarSet(VarSetExpr::new(var, initializer))
        } else {
            ExprKind::VarGet(VarGetExpr::new(var))
        };

        Expr::new(expr_kind)
    }

    fn parse_do(&mut self) -> Expr {
        // Consume "do" keyword
        self.expect(TokenType::Keyword(Keyword::Do));
        self.expect(TokenType::Line);

        self.parse_block()
    }

    fn parse_if(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::If));

        let cond = self.parse_expression();

        self.expect(TokenType::Keyword(Keyword::Then));
        self.expect(TokenType::Line);

        let mut then = vec![];
        while !self.match_(TokenType::Keyword(Keyword::End)) &&
            !self.match_(TokenType::Keyword(Keyword::Else)) {
            then.push(self.parse_top_level_expression());
            self.expect(TokenType::Line);
        }

        let expr_kind = if self.match_(TokenType::Keyword(Keyword::Else)) {
            self.consume();

            self.expect(TokenType::Line);

            let else_clause = self.parse_block().node.block().unwrap(); // TODO Unwrap

            ExprKind::IfElse(IfElseExpr::new(cond, BlockExpr::new(then), else_clause))
        } else {
            self.expect(TokenType::Keyword(Keyword::End));
            ExprKind::If(IfExpr::new(cond, then))
        };

        Expr::new(expr_kind)
    }

    fn parse_return(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::Return));

        let return_expr = if self.match_(TokenType::Line) {
            ReturnExpr::new(None)
        } else {
            ReturnExpr::new(Some(
                self.parse_top_level_expression()
            ))
        };

        Expr::new(ExprKind::Return(return_expr))
    }

    fn parse_block(&mut self) -> Expr {
        // TODO Check for single line expr: do print(10) end

        let mut exprs = vec![];
        while !self.match_(TokenType::Keyword(Keyword::End)) {
            exprs.push(self.parse_top_level_expression());
            self.expect(TokenType::Line);
        }

        self.expect(TokenType::Keyword(Keyword::End));

        Expr::new(ExprKind::Block(BlockExpr::new(exprs)))
    }

    pub fn match_(&mut self, token_type: TokenType) -> bool {
        self.peek_type() == token_type
    }

    fn peek_type(&self) -> TokenType {
        if self.tokens.len() == 0 {
            return TokenType::EOF;
        }
        self.peek().token_type
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.tokens.len() - 1]
    }

    pub fn expect(&mut self, expect: TokenType) -> Token<'a> {
        if self.peek_type() == expect {
            return self.consume();
        }
        panic!("Expected {:?}, got: {:?}", expect, self.peek_type());
    }

    pub fn consume(&mut self) -> Token<'a> {
        self.tokens.pop().unwrap()
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
}