use crate::lexer::lex_error::print_error_with_line;
use crate::lexer::lexer::lex;

mod lexer;

static LOGO: &str = r" ___  __                   ___       ________  ________   ________
|\  \|\  \                |\  \     |\   __  \|\   ___  \|\   ____\
\ \  \/  /|_  ____________\ \  \    \ \  \|\  \ \  \\ \  \ \  \___|
 \ \   ___  \|\____________\ \  \    \ \   __  \ \  \\ \  \ \  \  ___
  \ \  \\ \  \|____________|\ \  \____\ \  \ \  \ \  \\ \  \ \  \|\  \
   \ \__\\ \__\              \ \_______\ \__\ \__\ \__\\ \__\ \_______\
    \|__| \|__|               \|_______|\|__|\|__|\|__| \|__|\|_______|";

fn main() {
    const COMMANDS: [(&str, &str); 4] = [
        ("RUN", "run"),
        ("TEST", "test"),
        ("VERSION", "version"),
        ("HELP", "help"),
    ];
    let max_command_width = COMMANDS
        .iter()
        .map(|(command, _)| command.len())
        .max()
        .unwrap_or(0);

    match std::env::args().nth(1) {
        Some(args) => {
            let source_code = std::fs::read_to_string(&args).unwrap();
            match lex(source_code.as_str()) {
                Ok(tokens) => {
                    println!("{:?}", tokens);
                }
                Err(err) => {
                    print_error_with_line(err);
                }
            }
        }
        None => {
            println!("{LOGO}");
            println!("Usage: klang [command] [args]\n");
            for (command, description) in COMMANDS.iter() {
                print_command_with_description(command, description, max_command_width);
            }
        }
    }
}

fn print_command_with_description(command: &str, description: &str, column_width: usize) {
    println!(
        "{:<width$}{}",
        command,
        description,
        width = column_width + 5
    );
}
