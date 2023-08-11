use pest::Parser;
use pest_derive::Parser;

mod vm;


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PestGrammarParser;

fn main() {
    let input = std::fs::read_to_string("fib.kl").unwrap();
    match PestGrammarParser::parse(Rule::program, &input) {
        Ok(pairs) => {
            for pair in pairs {
                process_statement(pair.into_inner());
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn process_statement(pairs: pest::iterators::Pairs<Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::VAR_DECLARATION => process_var_declaration(pair.into_inner()),
            Rule::STATEMENT => process_statement(pair.into_inner()),
            Rule::SEMICOLON => (),
            Rule::EOI => (),
            _ => println!("Unknown rule: {:?}", pair.as_rule()),
        }
    }
}

fn process_var_declaration(pairs: pest::iterators::Pairs<Rule>) {
    let mut identifier = String::new();
    let mut expression = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::IDENTIFIER => identifier = pair.as_str().to_string(),
            Rule::EXPRESSION => expression = pair.as_str().to_string(),
            Rule::EQUALS => (),
            Rule::SEMICOLON => println!("End declaration"),
            _ => println!("{}", pair.as_str().to_string()),
        }
    }

    println!("Variable: {}", identifier);
    println!("Value: {}", expression);
}
