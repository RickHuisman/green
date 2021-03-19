use crate::scanner::lexer::Lexer;
use crate::scanner::token::Token;
use crate::parser::rule::{GrammarRules, Precedence};
use std::borrow::Borrow;
use crate::parser::ast::expr::Expr;
use std::thread::current;

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens }
    }

    fn current(&mut self) -> Option<&Token<'a>> {
        if self.tokens.len() < self.tokens.len() - 1 {
            return None;
        }
        self.tokens.get(self.tokens.len() - 1) // TODO Works ???
    }

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

pub struct EvalParser<'a> {
    parser: Parser<'a>,
    grammar: GrammarRules,
}

impl<'a> EvalParser<'a> {
    pub fn parse(source: &str) -> Vec<Expr> {
        let mut tokens = Lexer::parse(source);
        tokens.reverse();

        let mut parser = Parser { tokens };
        let mut eval_parser = EvalParser { parser, grammar: GrammarRules {} };

        let mut exprs = Vec::new();
        while !eval_parser.parser.is_empty() {
            exprs.push(eval_parser.parse_statement());
        }

        exprs
    }

    // Eval doesn't have statements but "top-level" expressions.
    fn parse_statement(&mut self) -> Expr {
        match self.parser.peek() {
            _ => self.parse_expression()
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::None)
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Expr {
        // Prefix
        let token = self.parser.next();

        if let Some(prefix) = self.grammar.get_prefix_rule(&token) {
            let mut left = prefix.parse(self, token);

            // Infix
            if !self.parser.is_empty() {
                self.parse_infix(left, precedence as u8)
            } else {
                left
            }
        } else {
            panic!("Cannot parse an expression.");
        }
    }

    fn parse_infix(&mut self, left: Expr, precedence: u8) -> Expr {
        let mut test = left;

        loop {
            if !self.parser.is_empty() {
                let current_precedence = self.grammar.get_precedence(&self.parser.current().unwrap()) as u8;

                if precedence < current_precedence {
                    let token = self.parser.next();
                    if let Some(infix) = self.grammar.get_infix_rule(&token) {
                        test = infix.parse(self, test, token);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        test
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"do 5 + 10 end"#;

        let exprs = EvalParser::parse(input);
        println!("{:?}", exprs);
    }
}