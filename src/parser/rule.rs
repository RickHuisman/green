use crate::scanner::token::{TokenType, Token};
use crate::parser::parser::EvalParser;
use std::collections::HashMap;
use crate::parser::ast::expr::{Expr, Literal, ExprKind, BinaryExpr, BinaryOperator};
use std::marker::PhantomData;
use crate::parser::ast::expr::ExprKind::Binary;

pub trait PrefixParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr;
}

pub trait InfixParser {
    fn parse<'a>(&self, parser: &mut EvalParser, left: Expr, token: Token<'a>) -> Expr;
    fn get_precedence(&self) -> Precedence;
}

pub struct GrammarRules {}

impl GrammarRules {
    pub fn new() -> Self {
        GrammarRules {}
    }

    pub fn get_prefix_rule(&self, token: &Token) -> Option<Box<dyn PrefixParser>> {
        let mut map = HashMap::new();
        map.insert(TokenType::Number, LiteralParser {});

        println!("{:?}", token);

        Some(Box::new(map[&token.token_type]))
    }

    pub fn get_infix_rule(&self, token: &Token) -> Option<Box<dyn InfixParser>> {
        let mut map = HashMap::new();
        map.insert(TokenType::Plus, InfixOperatorParser::new(Precedence::Term));
        map.insert(TokenType::Minus, InfixOperatorParser::new(Precedence::Term));

        map.insert(TokenType::Star, InfixOperatorParser::new(Precedence::Factor));
        map.insert(TokenType::Slash, InfixOperatorParser::new(Precedence::Factor));

        map.insert(TokenType::EqualEqual, InfixOperatorParser::new(Precedence::Equality));
        map.insert(TokenType::BangEqual, InfixOperatorParser::new(Precedence::Equality));
        map.insert(TokenType::GreaterThan, InfixOperatorParser::new(Precedence::Comparison));
        map.insert(TokenType::GreaterThanEqual, InfixOperatorParser::new(Precedence::Comparison));
        map.insert(TokenType::LessThan, InfixOperatorParser::new(Precedence::Comparison));
        map.insert(TokenType::LessThanEqual, InfixOperatorParser::new(Precedence::Comparison));

        Some(Box::new(map[&token.token_type]))
    }

    pub fn get_precedence(&self, token: &Token) -> Precedence {
        let mut precedence = Precedence::None;

        if let Some(parser) = self.get_infix_rule(token) {
            precedence = parser.get_precedence();
        }

        precedence
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Precedence {
    None = 0,
    Assignment = 1,
    // =
    Equality = 4,
    // == !=
    Comparison = 5,
    // < > <= >=
    Term = 6,
    // + -
    Factor = 7,
    // * /
    Unary = 8, // ! -
}

#[derive(Copy, Clone)]
struct LiteralParser {}

impl PrefixParser for LiteralParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr {
        let op = match token.token_type {
            TokenType::Number => {
                Literal::Number(token.source.parse::<f64>().unwrap())
            }
            TokenType::String => Literal::String(token.source.to_string()), // TODO
            _ => panic!("TODO") // TODO
        };
        Expr { node: ExprKind::Literal(op) }
    }
}

#[derive(Copy, Clone)]
struct InfixOperatorParser {
    precedence: Precedence,
}

impl InfixOperatorParser {
    pub fn new(precedence: Precedence) -> Self {
        InfixOperatorParser { precedence }
    }
}

impl InfixParser for InfixOperatorParser {
    fn parse<'a>(&self, parser: &mut EvalParser, left: Expr, token: Token<'a>) -> Expr {
        // Assume left associativity.
        let right = parser.parse_precedence(self.precedence);

        let binary = BinaryExpr::new(
            left,
            right,
            BinaryOperator::from_token(token.token_type),
        );

        Expr::new(ExprKind::Binary(binary))
    }

    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}