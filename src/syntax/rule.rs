use crate::error::ParserError;
use crate::syntax::expr::{
    ArrayExpr, BinaryExpr, BinaryOperator, CallExpr, Expr, ExprKind, GetExpr, GroupingExpr,
    LiteralExpr, SetExpr, SubscriptExpr, UnaryExpr, UnaryOperator, VarGetExpr, VarSetExpr,
    Variable,
};
use crate::syntax::parser::GreenParser;
use crate::syntax::token::{Keyword, Token, TokenType};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, ParserError>;

pub trait PrefixParser {
    fn parse<'a>(&self, parser: &mut GreenParser, token: Token<'a>) -> Result<Expr>;
}

pub trait InfixParser {
    fn parse<'a>(&self, parser: &mut GreenParser, left: Expr, token: Token<'a>) -> Result<Expr>;
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

    let mut map5 = HashMap::new();
    map5.insert(TokenType::LeftBracket, ArrayParser {});

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
                } else {
                    if let Some(token_type) = map5.get(token_type) {
                        Some(Box::new(*token_type))
                    } else {
                        None
                    }
                }
            }
        }
    }
}

pub fn get_infix_rule(token_type: &TokenType) -> Option<Box<dyn InfixParser>> {
    let mut map = HashMap::new();
    map.insert(TokenType::Plus, InfixOperatorParser::new(Precedence::Term));
    map.insert(TokenType::Minus, InfixOperatorParser::new(Precedence::Term));

    map.insert(
        TokenType::Star,
        InfixOperatorParser::new(Precedence::Factor),
    );
    map.insert(
        TokenType::Slash,
        InfixOperatorParser::new(Precedence::Factor),
    );

    map.insert(
        TokenType::EqualEqual,
        InfixOperatorParser::new(Precedence::Equality),
    );
    map.insert(
        TokenType::BangEqual,
        InfixOperatorParser::new(Precedence::Equality),
    );
    map.insert(
        TokenType::GreaterThan,
        InfixOperatorParser::new(Precedence::Comparison),
    );
    map.insert(
        TokenType::GreaterThanEqual,
        InfixOperatorParser::new(Precedence::Comparison),
    );
    map.insert(
        TokenType::LessThan,
        InfixOperatorParser::new(Precedence::Comparison),
    );
    map.insert(
        TokenType::LessThanEqual,
        InfixOperatorParser::new(Precedence::Comparison),
    );

    let mut map2 = HashMap::new();
    map2.insert(TokenType::LeftParen, CallParser::new());

    let mut map3 = HashMap::new();
    map3.insert(TokenType::LeftBracket, SubscriptParser::new());

    let mut map4 = HashMap::new();
    map4.insert(TokenType::Dot, DotParser::new());

    if let Some(token_type) = map.get(&token_type) {
        Some(Box::new(*token_type))
    } else {
        if let Some(token_type) = map2.get(&token_type) {
            Some(Box::new(*token_type))
        } else {
            if let Some(token_type) = map3.get(&token_type) {
                Some(Box::new(*token_type))
            } else {
                if let Some(token_type) = map4.get(&token_type) {
                    Some(Box::new(*token_type))
                } else {
                    None
                }
            }
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
    // or
    Or = 2,
    // and
    And = 3,
    // =
    Equality = 4,
    // == !=
    Comparison = 5,
    // < > <= >=
    Term = 6,
    // + -
    Factor = 7,
    // * /
    Unary = 8,      // ! -
    Call = 9,       // x()
    Subscript = 10, // [] .
}

#[derive(Copy, Clone)]
struct LiteralParser;

impl PrefixParser for LiteralParser {
    fn parse<'a>(&self, parser: &mut GreenParser, token: Token<'a>) -> Result<Expr> {
        let op = match token.token_type {
            TokenType::Number => LiteralExpr::Number(token.source.parse::<f64>().unwrap()),
            TokenType::String => LiteralExpr::String(token.source.to_string()), // TODO
            TokenType::Keyword(Keyword::True) => LiteralExpr::True,
            TokenType::Keyword(Keyword::False) => LiteralExpr::False,
            _ => panic!("No rule for token: {:?}", token),
        };
        Ok(Expr::new(ExprKind::Literal(op)))
    }
}

#[derive(Copy, Clone)]
struct GroupingParser;

impl PrefixParser for GroupingParser {
    fn parse<'a>(&self, parser: &mut GreenParser, token: Token<'a>) -> Result<Expr> {
        let expr = parser.parse_expression()?;
        parser.expect(TokenType::RightParen)?;
        Ok(Expr::new(ExprKind::Grouping(GroupingExpr::new(expr))))
    }
}

#[derive(Copy, Clone)]
struct IdentifierParser;

impl PrefixParser for IdentifierParser {
    fn parse<'a>(&self, parser: &mut GreenParser, token: Token<'a>) -> Result<Expr> {
        let var = Variable::new(token.source.to_string());

        Ok(if parser.match_(TokenType::Equal)? {
            let initializer = parser.parse_expression()?;

            Expr::var_set(VarSetExpr::new(var, initializer))
        } else {
            Expr::var_get(VarGetExpr::new(var))
        })
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
    fn parse<'a>(&self, parser: &mut GreenParser, left: Expr, token: Token<'a>) -> Result<Expr> {
        // Assume left associativity.
        let right = parser.parse_precedence(self.precedence)?;

        let binary = BinaryExpr::new(
            left,
            right,
            BinaryOperator::from_token(token.token_type).unwrap(),
        );

        Ok(Expr::new(ExprKind::Binary(binary)))
    }

    fn get_precedence(&self) -> Precedence {
        self.precedence
    }
}

#[derive(Copy, Clone)]
struct CallParser;

impl CallParser {
    pub fn new() -> Self {
        CallParser {}
    }
}

impl InfixParser for CallParser {
    fn parse<'a>(&self, parser: &mut GreenParser, left: Expr, token: Token<'a>) -> Result<Expr> {
        let mut args = vec![];
        if !parser.check(TokenType::RightParen)? {
            args.push(parser.parse_expression()?);
            while parser.match_(TokenType::Comma)? {
                parser.consume()?;
                args.push(parser.parse_expression()?);
            }
        }
        parser.expect(TokenType::RightParen)?;

        Ok(Expr::new(ExprKind::Call(CallExpr::new(left, args))))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Call
    }
}

#[derive(Copy, Clone)]
struct SubscriptParser;

impl SubscriptParser {
    pub fn new() -> Self {
        SubscriptParser {}
    }
}

impl InfixParser for SubscriptParser {
    fn parse<'a>(&self, parser: &mut GreenParser, left: Expr, token: Token<'a>) -> Result<Expr> {
        let index = parser.parse_precedence(Precedence::Or)?;
        parser.expect(TokenType::RightBracket)?;

        let expr = if parser.match_(TokenType::Equal)? {
            parser.consume()?;
            Some(parser.parse_expression()?)
        } else {
            None
        };

        Ok(Expr::new(ExprKind::Subscript(SubscriptExpr::new(
            left, index, expr,
        ))))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Subscript
    }
}

#[derive(Copy, Clone)]
struct UnaryParser;

impl UnaryParser {
    pub fn new() -> Self {
        UnaryParser {}
    }
}

impl PrefixParser for UnaryParser {
    fn parse<'a>(&self, parser: &mut GreenParser, token: Token<'a>) -> Result<Expr> {
        let operator_type = token.token_type;

        let expr = parser.parse_expression()?;

        let op = match operator_type {
            TokenType::Minus => UnaryOperator::Negate,
            TokenType::Bang => UnaryOperator::Not,
            _ => panic!("TODO"), // TODO
        };

        Ok(Expr::new(ExprKind::Unary(UnaryExpr::new(expr, op))))
    }
}

#[derive(Copy, Clone)]
struct ArrayParser;

impl ArrayParser {
    pub fn new() -> Self {
        ArrayParser {}
    }
}

impl PrefixParser for ArrayParser {
    fn parse<'a>(&self, parser: &mut GreenParser, token: Token<'a>) -> Result<Expr> {
        let mut exprs = vec![];

        if !parser.match_(TokenType::RightBracket)? {
            loop {
                if parser.match_(TokenType::RightBracket)? {
                    // trailing comma case
                    break;
                }

                let expr = parser.parse_precedence(Precedence::Or)?;
                exprs.push(expr);

                // TODO Max items in array
                // if (itemCount == UINT8_COUNT) {
                //     error("Cannot have more than 256 items in a array literal.");
                // }

                if parser.match_(TokenType::Comma)? {
                    parser.consume()?;
                }
            }
        }

        parser.expect(TokenType::RightBracket)?;

        Ok(Expr::new(ExprKind::Array(ArrayExpr::new(Some(exprs)))))
    }
}

#[derive(Copy, Clone)]
struct DotParser;

impl DotParser {
    pub fn new() -> Self {
        DotParser {}
    }
}

impl InfixParser for DotParser {
    fn parse<'a>(&self, parser: &mut GreenParser, left: Expr, token: Token<'a>) -> Result<Expr> {
        let property_token = parser.expect(TokenType::Identifier)?;
        let property = property_token.source;

        if parser.match_(TokenType::Equal)? {
            let value = parser.parse_expression()?;
            Ok(Expr::set_property(SetExpr::new(
                left,
                value,
                property.to_string(),
            )))
        } else {
            Ok(Expr::get_property(GetExpr::new(left, property.to_string())))
        }

        // uint8_t name = identifierConstant(&parser.previous);
        //
        // if (canAssign && match(TOKEN_EQUAL)) {
        //     expression();
        //     emitBytes(OP_SET_PROPERTY, name);
        // } else {
        //     emitBytes(OP_GET_PROPERTY, name);
        // }
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Call
    }
}
