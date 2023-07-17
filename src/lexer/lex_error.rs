pub enum LexError {
    InvalidEscapeSequence(char, String),
    UnexpectedBackslash(String),
}

impl LexError {
    pub fn msg(&self) -> String {
        match self {
            LexError::InvalidEscapeSequence(ch, _line) => {
                format!("Invalid escape sequence: '{}'", ch)
            }
            LexError::UnexpectedBackslash(_line) => "Unexpected backslash".to_string(),
        }
    }

    pub fn line(&self) -> &str {
        match self {
            LexError::InvalidEscapeSequence(_, line) => line,
            LexError::UnexpectedBackslash(line) => line,
        }
    }
}
