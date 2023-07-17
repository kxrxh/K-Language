use once_cell::sync::Lazy;
use regex::Regex;

use crate::lexer::lex_error::LexError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Integer(i32),
    Float(f64),
    StringLiteral(String),
    Operator(String),
    Parenthesis(char),
    Semicolon,
    Colon,
    Comma,
    CurlyBracketOpen,
    CurlyBracketClose,
    Func,
    Return,
}

static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(?x)
        \b(let|func|return)\b
        |\b([a-zA-Z_][a-zA-Z0-9_]*)\b
        |\b(\d+\.\d+|\d+)\b
        |("[^"\\]*(?:\\.[^"\\]*)*")
        |[-+*/();=]|->|:|,|\{|}
    "#,
    )
    .unwrap()
});

pub fn lex(source_code: &str) -> Result<Vec<Token>, LexError> {
    TOKEN_REGEX
        .find_iter(source_code)
        .map(|mat| {
            let token = mat.as_str();
            match token {
                "let" => Ok(Token::Let),
                "func" => Ok(Token::Func),
                "return" => Ok(Token::Return),
                ";" => Ok(Token::Semicolon),
                "=" | "+" | "*" | "-" | "/" | "<<" | ">>" => Ok(Token::Operator(token.to_string())),
                ":" => Ok(Token::Colon),
                "," => Ok(Token::Comma),
                "{" => Ok(Token::CurlyBracketOpen),
                "}" => Ok(Token::CurlyBracketClose),
                "(" => Ok(Token::Parenthesis('(')),
                ")" => Ok(Token::Parenthesis(')')),
                _ => {
                    if let Some(number_token) = parse_number(token) {
                        Ok(number_token)
                    } else if token.starts_with('"') && token.ends_with('"') {
                        let unescaped = &token[1..token.len() - 1];
                        let mut escaped = String::new();
                        let mut chars = unescaped.chars();
                        let mut escape_next = false;

                        while let Some(ch) = chars.next() {
                            if escape_next {
                                match ch {
                                    '\\' | '"' | 'n' | 't' | 'r' | '\'' => escaped.push(ch),
                                    _ => {
                                        return Err(LexError::InvalidEscapeSequence(
                                            ch,
                                            token.to_string(),
                                        ));
                                    }
                                }
                                escape_next = false;
                            } else if ch == '\\' {
                                escape_next = true;
                            } else {
                                escaped.push(ch);
                            }
                        }
                        if escape_next {
                            return Err(LexError::UnexpectedBackslash(token.to_string()));
                        }
                        Ok(Token::StringLiteral(escaped))
                    } else {
                        Ok(Token::Identifier(token.to_string()))
                    }
                }
            }
        })
        .collect()
}

/// Parse a number
///
/// # Arguments
///
/// * `token`: string representing a number
///
/// returns: Option<Token>
///
fn parse_number(token: &str) -> Option<Token> {
    if let Ok(int_value) = token.parse::<i32>() {
        return Some(Token::Integer(int_value));
    }

    if let Ok(float_value) = token.parse::<f64>() {
        return Some(Token::Float(float_value));
    }

    None
}
