use crate::scanner::token::{TokenType, Token};
use crate::parser::parser::Parser;
use std::collections::HashMap;
use crate::parser::ast::expr::{Expr, Literal, ExprKind};

pub trait PrefixParser {
    fn parse<'a>(&self, parser: &mut Parser, token: Token<'a>) -> Expr;
}

trait InfixParser {
    fn new() -> Self;
    fn parse<'a>(&self, parser: &mut Parser, left: Expr, token: Token<'a>) -> Expr;
}

pub struct GrammarRules {
    // infix_rules: HashMap<TokenType<'a>, LiteralParser>,
}

impl GrammarRules {
    pub fn get_prefix_rule(token: &Token) -> Box<dyn PrefixParser> {
        // let mut rules: HashMap<TokenType<'a>, &PrefixParser> = HashMap::new();
        let mut map = HashMap::new();
        map.insert(TokenType::Number, LiteralParser {});
        // map.insert(TokenType::Plus, );

        Box::new(map[&token.token_type])
    }
}

enum Precedence {
    None = 1,
}

#[derive(Copy, Clone)]
struct LiteralParser {
}

impl PrefixParser for LiteralParser {
    fn parse<'a>(&self, parser: &mut Parser, token: Token<'a>) -> Expr {
        let op = match token.token_type {
            TokenType::Number => Literal::Number(10 as f64), // TODO
            TokenType::String => Literal::String("".to_string()), // TODO
            _ => panic!("TODO") // TODO
        };
        Expr { node: ExprKind::Literal(op) }
    }
}

// pub struct ParseRule<'a> {
//     token_type: TokenType<'a>,
//     prefix: Box<Fn() -> usize>,
//     infix: Box<Fn() -> usize>,
//     precedence: Precedence,
// }

// pub struct ParseRules {
//
// }

// impl ParseRules {
//     pub fn test<'a>() -> Vec<ParseRule<'a>> {
//         vec![
//             ParseRule { token_type: TokenType::LeftParen, prefix: (), infix: (), precedence: Precedence::None }
//         ]
//     }
// }