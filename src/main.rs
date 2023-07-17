mod lexer;
mod lexer_tests;

fn main() {
    let f = std::fs::read_to_string("file.txt").unwrap();
    println!("{:?}", lexer::lex(&f));
}
