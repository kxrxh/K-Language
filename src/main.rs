mod cli;
mod lexer;
mod parser;

fn main() {
    let input = std::fs::read_to_string("simple.kl").unwrap();
    parse_code(&input);
}

fn parse_code(code: &str) {
    let pairs = lexer::parse_code(code);
    match pairs {
        Some(pairs) => {
            parser::run_parser(pairs);
        }
        None => {
            eprintln!("Error parsing code");
        }
    }
}
