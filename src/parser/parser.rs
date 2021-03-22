use crate::scanner::lexer::Lexer;
use crate::scanner::token::{Token, TokenType, Keyword, Position};
use crate::parser::rule::{GrammarRules, Precedence};
use crate::parser::ast::expr::{Expr, ExprKind, BlockExpr, LiteralExpr, Variable, VarSetExpr, VarGetExpr, VarAssignExpr};
use crate::parser::ast::expr::ExprKind::{Literal, Block};
use crate::scanner::token::TokenType::Line;
use std::any::Any;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens }
    }

    fn match_(&mut self, token_type: TokenType) -> bool { // TODO match_ name
        self.peek_type() == token_type
    }

    fn peek_type(&mut self) -> TokenType {
        if self.tokens.len() == 0 {
            return TokenType::EOF;
        }

        self.tokens.get(self.tokens.len() - 1)
            .unwrap()
            .token_type
    }

    fn peek(&mut self) -> &Token<'a> {
        self.tokens.get(self.tokens.len() - 1).unwrap()
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

pub struct EvalParser<'a> {
    pub parser: Parser<'a>,
    grammar: GrammarRules,
}

impl<'a> EvalParser<'a> {
    pub fn parse(source: &str) -> Vec<Expr> {
        let mut tokens = Lexer::parse(source);
        tokens.reverse();

        let mut eval_parser = EvalParser {
            parser: Parser::new(tokens),
            grammar: GrammarRules {},
        };

        let mut exprs = vec![];
        while !eval_parser.parser.match_(TokenType::EOF) {
            // Consume lines till a non line token is found
            while eval_parser.parser.peek_type() == TokenType::Line {
                eval_parser.parser.consume();
            }

            exprs.push(eval_parser.parse_top_level_expression());

            if !eval_parser.parser.match_(TokenType::EOF) {
                eval_parser.parser.expect(TokenType::Line);
            }
        }

        exprs
    }

    // Eval doesn't have statements but "top-level" expressions.
    fn parse_top_level_expression(&mut self) -> Expr {
        match self.parser.peek_type() {
            TokenType::Keyword(Keyword::Print) => self.parse_print(),
            TokenType::Keyword(Keyword::Var) => self.declare_var(),
            TokenType::Keyword(Keyword::Do) => self.parse_do(),
            _ => self.parse_expression()
        }
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::None)
    }

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Expr {
        // Prefix
        let token = self.parser.consume();

        if let Some(prefix) = self.grammar.get_prefix_rule(&token) {
            let mut left = prefix.parse(self, token);

            // Infix
            if !self.parser.is_empty() {
                self.parse_infix(left, precedence as u8)
            } else {
                left
            }
        } else {
            panic!("Unexpected token {:?}.", token); // TODO
        }
    }

    fn parse_infix(&mut self, left: Expr, precedence: u8) -> Expr {
        let mut test = left;

        loop {
            if !self.parser.is_empty() {
                let current_precedence = self.grammar.get_precedence(&self.parser.peek()) as u8;

                if precedence < current_precedence {
                    let token = self.parser.consume();
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

    fn parse_print(&mut self) -> Expr {
        self.parser.expect(TokenType::Keyword(Keyword::Print));
        Expr::new(ExprKind::Print(self.parse_expression()))
    }

    fn declare_var(&mut self) -> Expr {
        self.parser.expect(TokenType::Keyword(Keyword::Var));

        let identifier = self.parser.expect(TokenType::Identifier);
        let var = Variable::new(identifier.source.to_string());

        let mut initializer = Expr::new(ExprKind::Literal(LiteralExpr::Nil));

        // Var has initializer
        if self.parser.match_(TokenType::Equal) {
            // Pop '=' operator
            self.parser.consume();

            initializer = self.parse_expression();
        }

        Expr::new(ExprKind::VarAssign(VarAssignExpr::new(var, initializer)))
    }

    pub fn parse_var(&mut self, identifier: Token) -> Expr {
        let var = Variable::new(identifier.source.to_string());

        // Var has initializer
        if self.parser.match_(TokenType::Equal) {
            // Pop '=' operator
            self.parser.consume();

            let initializer = self.parse_expression();
            Expr::new(ExprKind::VarSet(VarSetExpr::new(var, initializer)))
        } else {
            Expr::new(ExprKind::VarGet(VarGetExpr::new(var)))
        }
    }

    fn parse_do(&mut self) -> Expr {
        self.parse_block()
    }

    fn parse_block(&mut self) -> Expr {
        // TODO Check for single line expr: do print(10) end

        // Consume "do" keyword
        self.parser.expect(TokenType::Keyword(Keyword::Do));
        self.parser.expect(TokenType::Line);

        let mut exprs = vec![];
        while !self.parser.match_(TokenType::Keyword(Keyword::End)) {
            exprs.push(self.parse_top_level_expression());
            self.parser.expect(TokenType::Line);
        }

        self.parser.expect(TokenType::Keyword(Keyword::End));

        Expr::new(ExprKind::Block(BlockExpr::new(exprs)))
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
}