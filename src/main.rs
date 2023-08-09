// use clap::Parser;
// use crate::lexer::lex_error::print_error_with_line;
// use crate::lexer::lexer::lex;

// mod lexer;
// mod cli;

// fn main() {
//     let args = cli::Args::parse();
//     args.get_subcommand().run();
//     // const COMMANDS: [(&str, &str); 4] = [
//     ("RUN", "run"),
//     ("TEST", "test"),
//     ("VERSION", "version"),
//     ("HELP", "help"),
// ];
// let max_command_width = COMMANDS
//     .iter()
//     .map(|(command, _)| command.len())
//     .max()
//     .unwrap_or(0);
//
// match std::env::args().nth(1) {
//     Some(args) => {
//         let source_code = std::fs::read_to_string(&args).unwrap();
//         match lex(source_code.as_str()) {
//             Ok(tokens) => {
//                 println!("{:?}", tokens);
//             }
//             Err(err) => {
//                 print_error_with_line(err);
//             }
//         }
//     }
//     None => {
//         println!("{LOGO}");
//         println!("Usage: klang [command] [args]\n");
//         for (command, description) in COMMANDS.iter() {
//             print_command_with_description(command, description, max_command_width);
//         }
//     }
// }
//}

// fn print_command_with_description(command: &str, description: &str, column_width: usize) {
//     println!(
//         "{:<width$}{}",
//         command,
//         description,
//         width = column_width + 5
//     );
// }

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PestGrammarParser;

fn main() {
    let input = r#"let var = "dsa""#;

    match PestGrammarParser::parse(Rule::program, input) {
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
            Rule::DECLARATION_WORD => println!("Begin declaration:"),
            Rule::EQUALS => (),
            Rule::SEMICOLON => println!("End declaration"),
            _ => println!("{}", pair.as_str().to_string()),
        }
    }

    println!("Variable: {}", identifier);
    println!("Value: {}", expression);
}
