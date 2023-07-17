use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

pub fn print_error_with_line(error: LexError) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let error_line = format!("Line {}: {}", error.line(), error.msg());

    // Set color specification for error message
    let mut error_color_spec = ColorSpec::new();
    error_color_spec.set_fg(Some(Color::Red)).set_bold(true);

    stdout.set_color(&error_color_spec).unwrap();
    writeln!(&mut stdout, "error: {}", error_line).unwrap();

    // Reset color specification
    stdout.set_color(&ColorSpec::new()).unwrap();

    process::exit(1);
}
