use crate::scanner::token::{TokenType, Token};
use crate::parser::parser::EvalParser;
use std::collections::HashMap;
use crate::parser::ast::expr::{Expr, LiteralExpr, ExprKind, BinaryExpr, BinaryOperator, GroupingExpr};

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
        // TODO Fix this mess.
        let mut map = HashMap::new();
        map.insert(TokenType::Number, LiteralParser {});
        map.insert(TokenType::String, LiteralParser {});

        let mut map2 = HashMap::new();
        map2.insert(TokenType::LeftParen, GroupingParser {});

        let mut map3 = HashMap::new();
        map3.insert(TokenType::Identifier, IdentifierParser {});

        if let Some(token_type) = map.get(&token.token_type) {
            Some(Box::new(map[&token.token_type]))
        } else {
            if let Some(token_type) = map2.get(&token.token_type) {
                Some(Box::new(map2[&token.token_type]))
            } else {
                if let Some(token_type) = map3.get(&token.token_type) {
                    Some(Box::new(map3[&token.token_type]))
                } else {
                    println!("No rule for token: {:?}", token);
                    None
                }
            }
        }
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

        if let Some(token_type) = map.get(&token.token_type) {
            Some(Box::new(*token_type))
        } else {
            println!("No rule for token: {:?}", token);
            None
        }
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
                LiteralExpr::Number(token.source.parse::<f64>().unwrap())
            }
            TokenType::String => LiteralExpr::String(token.source.to_string()), // TODO
            _ => panic!("TODO") // TODO
        };
        Expr::new(ExprKind::Literal(op))
    }
}

#[derive(Copy, Clone)]
struct GroupingParser {}

impl PrefixParser for GroupingParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr {
        let expr = parser.parse_expression();
        parser.parser.expect(TokenType::RightParen);
        Expr::new(ExprKind::Grouping(GroupingExpr::new(expr)))
    }
}

#[derive(Copy, Clone)]
struct IdentifierParser {}

impl PrefixParser for IdentifierParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr {
        parser.parse_var(token)
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