use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Integer(i32),
    Float(f64),
    StringLiteral(String), // New variant for strings
    Operator(String),
    Parenthesis(char),
    Semicolon,
}

pub fn lex(source_code: &str) -> Vec<Token> {
    let token_regex = Regex::new(r#"\b(let)\b|\b([a-zA-Z_][a-zA-Z0-9_]*)\b|\b(\d+\.\d+|\d+)\b|("[^"]*")|[+\-*/();=]"#).unwrap();
    let mut tokens = Vec::new();

    for captures in token_regex.captures_iter(source_code) {
        if let Some(capture) = captures.get(1) {
            let token = capture.as_str();
            let token_type = match token {
                "let" => Token::Let,
                _ => unreachable!(),
            };
            tokens.push(token_type);
        } else if let Some(capture) = captures.get(2) {
            let token = capture.as_str();
            tokens.push(Token::Identifier(token.to_string()));
        } else if let Some(capture) = captures.get(3) {
            let token = capture.as_str();
            let token_type = if token.contains('.') {
                Token::Float(token.parse().unwrap())
            } else {
                Token::Integer(token.parse().unwrap())
            };
            tokens.push(token_type);
        } else if let Some(capture) = captures.get(4) {
            let token = capture.as_str();
            let token_type = Token::StringLiteral(token[1..token.len() - 1].to_string()); // Remove surrounding quotes
            tokens.push(token_type);
        } else {
            let token = captures.get(0).unwrap().as_str();
            let token_type = match token {
                ";" => Token::Semicolon,
                "=" => Token::Operator(token.to_string()),
                _ => Token::Operator(token.to_string()),
            };
            tokens.push(token_type);
        }
    }

    tokens
}
