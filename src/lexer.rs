use once_cell::sync::Lazy;
use regex::Regex;

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
        |("[^"\\]*")
        |[-+*/();=]|->|:|,|\{|}
    "#,
    )
    .unwrap()
});

pub fn lex(source_code: &str) -> Vec<Token> {
    TOKEN_REGEX
        .find_iter(source_code)
        .map(|mat| {
            let token = mat.as_str();
            match token {
                "let" => Token::Let,
                "func" => Token::Func,
                "return" => Token::Return,
                ";" => Token::Semicolon,
                "=" | "+" | "*" | "-" | "/" | "<<" | ">>" => Token::Operator(token.to_string()),
                ":" => Token::Colon,
                "," => Token::Comma,
                "{" => Token::CurlyBracketOpen,
                "}" => Token::CurlyBracketClose,
                "(" => Token::Parenthesis('('),
                ")" => Token::Parenthesis(')'),
                _ => {
                    if let Some(number_token) = parse_number(token) {
                        number_token
                    } else if token.starts_with('"') && token.ends_with('"') {
                        Token::StringLiteral(token[1..token.len() - 1].to_string())
                    } else {
                        Token::Identifier(token.to_string())
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
