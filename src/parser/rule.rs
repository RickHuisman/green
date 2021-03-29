use crate::scanner::token::{TokenType, Token, Keyword};
use crate::parser::parser::EvalParser;
use std::collections::HashMap;
use crate::parser::ast::expr::{Expr, LiteralExpr, ExprKind, BinaryExpr, BinaryOperator, GroupingExpr, CallExpr, UnaryOperator, UnaryExpr};

pub trait PrefixParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr;
}

pub trait InfixParser {
    fn parse<'a>(&self, parser: &mut EvalParser, left: Expr, token: Token<'a>) -> Expr;
    fn get_precedence(&self) -> Precedence;
}

pub fn get_prefix_rule(token_type: &TokenType) -> Option<Box<dyn PrefixParser>> {
    // TODO Fix this mess.
    let mut map = HashMap::new();
    map.insert(TokenType::Number, LiteralParser {});
    map.insert(TokenType::String, LiteralParser {});
    map.insert(TokenType::Keyword(Keyword::True), LiteralParser {});
    map.insert(TokenType::Keyword(Keyword::False), LiteralParser {});

    let mut map2 = HashMap::new();
    map2.insert(TokenType::LeftParen, GroupingParser {});

    let mut map3 = HashMap::new();
    map3.insert(TokenType::Identifier, IdentifierParser {});

    let mut map4 = HashMap::new();
    map4.insert(TokenType::Bang, UnaryParser {});
    map4.insert(TokenType::Minus, UnaryParser {});

    if let Some(token_type) = map.get(token_type) {
        Some(Box::new(*token_type))
    } else {
        if let Some(token_type) = map2.get(token_type) {
            Some(Box::new(*token_type))
        } else {
            if let Some(token_type) = map3.get(token_type) {
                Some(Box::new(*token_type))
            } else {
                if let Some(token_type) = map4.get(token_type) {
                    Some(Box::new(*token_type))
                }
                else {
                    println!("No rule for token type: {:?}", token_type);
                    None
                }
            }
        }
    }
}

pub fn get_infix_rule(token_type: &TokenType) -> Option<Box<dyn InfixParser>> {
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

    let mut map2 = HashMap::new();
    map2.insert(TokenType::LeftParen, CallParser::new());

    if let Some(token_type) = map.get(&token_type) {
        Some(Box::new(*token_type))
    } else {
        if let Some(token_type) = map2.get(&token_type) {
            Some(Box::new(*token_type))
        } else {
            println!("No rule for token type: {:?}", token_type);
            None
        }
    }
}

pub fn get_precedence(token: &Token) -> Precedence {
    let mut precedence = Precedence::None;

    if let Some(parser) = get_infix_rule(&token.token_type) {
        precedence = parser.get_precedence();
    }

    precedence
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
    Call = 9, // x()
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
            TokenType::Keyword(Keyword::True) => LiteralExpr::True,
            TokenType::Keyword(Keyword::False) => LiteralExpr::False,
            _ => panic!("No rule for token: {:?}", token),
        };
        Expr::new(ExprKind::Literal(op))
    }
}

#[derive(Copy, Clone)]
struct GroupingParser {}

impl PrefixParser for GroupingParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr {
        let expr = parser.parse_expression();
        parser.expect(TokenType::RightParen);
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

#[derive(Copy, Clone)]
struct CallParser {}

impl CallParser {
    pub fn new() -> Self { CallParser {} }
}

impl InfixParser for CallParser {
    fn parse<'a>(&self, parser: &mut EvalParser, left: Expr, token: Token<'a>) -> Expr {
        let mut args = vec![];
        if !parser.match_(TokenType::RightParen) {

            args.push(parser.parse_expression());
            while parser.match_(TokenType::Comma) {
                parser.consume();
                args.push(parser.parse_expression());
            }
        }
        parser.expect(TokenType::RightParen);

        Expr::new(ExprKind::Call(
            CallExpr::new(left, args)
        ))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Call
    }
}

#[derive(Copy, Clone)]
struct UnaryParser {}

impl UnaryParser {
    pub fn new() -> Self { UnaryParser {} }
}

impl PrefixParser for UnaryParser {
    fn parse<'a>(&self, parser: &mut EvalParser, token: Token<'a>) -> Expr {
        // var operatorType = token.Type;
        //
        // var expr = Expression();
        //
        // var op = operatorType switch
        // {
        //     TokenType.Minus => UnaryOperator.Negate,
        //     TokenType.Bang => UnaryOperator.Not,
        //     _ => throw new Exception() // TODO
        // };
        //
        // return new UnaryExpression(op, expr);

        let operator_type = token.token_type;

        let expr = parser.parse_expression();

        let op = match operator_type {
            TokenType::Minus => UnaryOperator::Negate,
            TokenType::Bang => UnaryOperator::Not,
            _ => panic!("TODO"),
        };

        Expr::new(ExprKind::Unary(
            UnaryExpr::new(expr, op)
        ))
    }
}