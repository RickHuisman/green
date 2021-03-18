use crate::scanner::lexer::Lexer;
use crate::scanner::token::Token;
use crate::parser::rule::GrammarRules;
use std::borrow::Borrow;
use crate::parser::ast::expr::Expr;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn parse(source: &str) {
        let mut tokens = Lexer::parse(source);
        tokens.reverse();
        let mut parser = Parser { tokens }; // TODO Reverse???

        let mut exprs = Vec::new();
        while !parser.is_empty() {
            exprs.push(parser.parse_statement());
        }
    }

    // Eval doesn't have statements but "top-level" expressions.
    fn parse_statement(&mut self) -> Expr {
        match self.peek() {
            _ => self.parse_expression()
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_precedence(0)
    }

    fn parse_precedence(&mut self, precedence: usize) -> Expr {
        // Prefix
        let token = self.next();
        let prefix_rule = GrammarRules::get_prefix_rule(&token);
        prefix_rule.parse(self, token)

        // Expr { node: ExprKind::Literal(Literal::Number(10))}
    }

    // pub fn parse_binary(&mut self) -> Expr {
    //     Expr {}
    // }

    fn next(&mut self) -> Token<'a> {
        self.tokens.pop().unwrap() // TODO
    }

    fn peek(&mut self) -> &Token<'a> {
        &self.tokens[self.tokens.len() - 1]
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
        let input = "2 + 10";

        let exprs = Parser::parse(input);
        println!("{:?}", exprs);
    }
}