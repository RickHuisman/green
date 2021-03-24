use crate::scanner::lexer::Lexer;
use crate::scanner::token::{Token, TokenType, Keyword, Position};
use crate::parser::rule::{GrammarRules, Precedence};
use crate::parser::ast::expr::{Expr, ExprKind, BlockExpr, LiteralExpr, Variable, VarSetExpr, VarGetExpr, VarAssignExpr, IfExpr, IfElseExpr};
use crate::parser::ast::expr::ExprKind::{Literal, Block};
use crate::scanner::token::TokenType::Line;
use std::any::Any;
use std::borrow::Borrow;

pub struct EvalParser<'a> {
    tokens: Vec<Token<'a>>,
    grammar: GrammarRules,
}

impl<'a> EvalParser<'a> {
    pub fn parse(source: &str) -> Vec<Expr> {
        let mut tokens = Lexer::parse(source);
        tokens.reverse();

        let mut eval_parser = EvalParser {
            tokens,
            grammar: GrammarRules {},
        };

        let mut exprs = vec![];
        while !eval_parser.match_(TokenType::EOF) {
            // Consume lines till a non line token is found
            while eval_parser.peek_type() == TokenType::Line {
                eval_parser.consume();
            }

            exprs.push(eval_parser.parse_top_level_expression());

            if !eval_parser.match_(TokenType::EOF) {
                eval_parser.expect(TokenType::Line);
            }
        }

        exprs
    }

    // Eval doesn't have statements but "top-level" expressions.
    fn parse_top_level_expression(&mut self) -> Expr {
        match self.peek_type() {
            TokenType::Keyword(Keyword::Print) => self.parse_print(),
            TokenType::Keyword(Keyword::Var) => self.declare_var(),
            TokenType::Keyword(Keyword::Do) => self.parse_do(),
            TokenType::Keyword(Keyword::If) => self.parse_if(),
            _ => self.parse_expression()
        }
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::None)
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Expr {
        // Prefix
        let token = self.consume();

        if let Some(prefix) = self.grammar.get_prefix_rule(&token) {
            let mut left = prefix.parse(self, token);

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

        while precedence < self.grammar.get_precedence(self.peek()) as u8 {
            let token = self.consume();
            if let Some(infix) = self.grammar.get_infix_rule(&token) {
                infix2 = infix.parse(self, infix2, token);
            }
        }

        infix2
    }

    fn parse_print(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::Print));
        Expr::new(ExprKind::Print(self.parse_expression()))
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
        if self.match_(TokenType::Equal) {
            // Pop '=' operator
            self.consume();

            let initializer = self.parse_expression();
            Expr::new(ExprKind::VarSet(VarSetExpr::new(var, initializer)))
        } else {
            Expr::new(ExprKind::VarGet(VarGetExpr::new(var)))
        }
    }

    fn parse_do(&mut self) -> Expr {
        self.parse_block()
    }

    fn parse_if(&mut self) -> Expr {
        self.expect(TokenType::Keyword(Keyword::If));

        let cond = self.parse_expression();

        self.expect(TokenType::Keyword(Keyword::Then));
        self.expect(TokenType::Line);

        // TODO Multiple exprs in then block???
        let then = self.parse_top_level_expression();

        self.expect(TokenType::Line);

        let expr_kind = if self.match_(TokenType::Keyword(Keyword::Else)) {
            self.consume();

            self.expect(TokenType::Line);

            // TODO Multiple exprs in then block???
            let else_clause = self.parse_top_level_expression();

            self.expect(TokenType::Line);

            ExprKind::IfElse(IfElseExpr::new(cond, then, else_clause))
        } else {
            ExprKind::If(IfExpr::new(cond, then))
        };

        self.expect(TokenType::Keyword(Keyword::End));

        Expr::new(expr_kind)
    }

    fn parse_block(&mut self) -> Expr {
        // TODO Check for single line expr: do print(10) end

        // Consume "do" keyword
        self.expect(TokenType::Keyword(Keyword::Do));
        self.expect(TokenType::Line);

        let mut exprs = vec![];
        while !self.match_(TokenType::Keyword(Keyword::End)) {
            exprs.push(self.parse_top_level_expression());
            self.expect(TokenType::Line);
        }

        self.expect(TokenType::Keyword(Keyword::End));

        Expr::new(ExprKind::Block(BlockExpr::new(exprs)))
    }

    fn match_(&mut self, token_type: TokenType) -> bool {
        self.peek_type() == token_type
    }

    fn peek_type(&self) -> TokenType {
        if self.tokens.len() == 0 {
            return TokenType::EOF;
        }

        self.tokens[self.tokens.len() - 1].token_type
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.tokens.len() - 1]
    }

    pub fn expect(&mut self, expect: TokenType) -> Token<'a> {
        if self.peek_type() == expect {
            self.consume()
        } else {
            panic!("Expected {:?}, got: {:?}", expect, self.peek_type());
        }
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
        "#;

        let exprs = EvalParser::parse(input);
        println!("{:?}", exprs);
    }
}