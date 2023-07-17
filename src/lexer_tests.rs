#[cfg(test)]
mod tests {
    use crate::lexer::{lex, Token};

    #[test]
    fn test_lex_integer() {
        let source_code = "let x = 123;";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Operator("=".to_string()),
            Token::Integer(123),
            Token::Semicolon,
        ];
        assert_eq!(lex(source_code), expected_tokens);
    }

    #[test]
    fn test_lex_float() {
        let source_code = "let y = 3.14;";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Operator("=".to_string()),
            Token::Float(3.14),
            Token::Semicolon,
        ];
        assert_eq!(lex(source_code), expected_tokens);
    }

    #[test]
    fn test_lex_string() {
        let source_code = r#"let z = "hello";"#;
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("z".to_string()),
            Token::Operator("=".to_string()),
            Token::StringLiteral("hello".to_string()),
            Token::Semicolon,
        ];
        assert_eq!(lex(source_code), expected_tokens);
    }

    #[test]
    fn test_lex_function_declaration() {
        let source_code = r#"
            let x = 10;
            let y = "data";
            let c = x + y;

            func add(x, y) {
                return x + y;
            }
        "#;

        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Operator("=".to_string()),
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Operator("=".to_string()),
            Token::StringLiteral("data".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("c".to_string()),
            Token::Operator("=".to_string()),
            Token::Identifier("x".to_string()),
            Token::Operator("+".to_string()),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Func,
            Token::Identifier("add".to_string()),
            Token::Parenthesis('('.to_string().parse().unwrap()),
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::Parenthesis(')'.to_string().parse().unwrap()),
            Token::CurlyBracketOpen,
            Token::Return,
            Token::Identifier("x".to_string()),
            Token::Operator("+".to_string()),
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::CurlyBracketClose,
        ];

        assert_eq!(lex(source_code), expected_tokens);
    }
    #[test]
    fn test_lex_operators() {
        let source_code = "let a = 5 + 3 * 2 - 4 / 2;";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("a".to_string()),
            Token::Operator("=".to_string()),
            Token::Integer(5),
            Token::Operator("+".to_string()),
            Token::Integer(3),
            Token::Operator("*".to_string()),
            Token::Integer(2),
            Token::Operator("-".to_string()),
            Token::Integer(4),
            Token::Operator("/".to_string()),
            Token::Integer(2),
            Token::Semicolon,
        ];
        assert_eq!(lex(source_code), expected_tokens);
    }

    #[test]
    fn test_lex_function_call() {
        let source_code = "let result = add(2, 3);";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Operator("=".to_string()),
            Token::Identifier("add".to_string()),
            Token::Parenthesis('('.to_string().parse().unwrap()),
            Token::Integer(2),
            Token::Comma,
            Token::Integer(3),
            Token::Parenthesis(')'.to_string().parse().unwrap()),
            Token::Semicolon,
        ];
        assert_eq!(lex(source_code), expected_tokens);
    }
}
