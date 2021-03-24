use crate::scanner::token::{Token, TokenType};
use crate::scanner::lexer::Lexer;

/// Cleans a sequence of tokens into a token sequence of meaningful tokens.
/// Tokens that are removed from the sequence:
/// - Comments
/// - Unessential lines
pub fn morph(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut morphed = vec![];

    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token.token_type {
            TokenType::LineComment => {
                // Ignore comments.
            },
            TokenType::Line => {
                if morphed.is_empty() {
                    morphed.push(token);
                } else {
                    let last_token_type = morphed.last().unwrap().token_type;
                    if last_token_type != TokenType::Line {
                        morphed.push(token);
                    }
                }
            }
            _ => morphed.push(token)
        }
    }

    morphed.reverse();

    morphed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morph_comments() {
        let input = r#"
        # This is a test!
        print(10)

        "#;
        let tokens = Lexer::parse(input);
        for token in &tokens {
            println!("{:?}", token);
        }

        println!("Morph");

        let tokens2 = morph(tokens);
        for token in tokens2 {
            println!("{:?}", token);
        }
    }
}
