use clap::{Parser, Subcommand};

static LOGO: &str = r" ___  __                   ___       ________  ________   ________
|\  \|\  \                |\  \     |\   __  \|\   ___  \|\   ____\
\ \  \/  /|_  ____________\ \  \    \ \  \|\  \ \  \\ \  \ \  \___|
 \ \   ___  \|\____________\ \  \    \ \   __  \ \  \\ \  \ \  \  ___
  \ \  \\ \  \|____________|\ \  \____\ \  \ \  \ \  \\ \  \ \  \|\  \
   \ \__\\ \__\              \ \_______\ \__\ \__\ \__\\ \__\ \_______\
    \|__| \|__|               \|_______|\|__|\|__|\|__| \|__|\|_______|";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = LOGO)]
pub(crate) struct Args {
    #[command(subcommand)]
    subcommand: Command,
}

impl Args {
    pub fn get_subcommand(&self) -> &Command {
        &self.subcommand
    }
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Run(RunArgs),
    Compile(CompileArgs),
    Check,
    Test,
    Version,
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Run(args) => {
                // Access and use RunArgs here
            }
            Command::Check => {
                // Handle Check subcommand
            }
            Command::Test => {
                // Handle Test subcommand
            }
            Command::Version => {
                println!(
                    "{name} {version}",
                    name = env!("CARGO_PKG_NAME"),
                    version = env!("CARGO_PKG_VERSION")
                );
            }
        }
    }
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    #[arg(help = "Input klang source code file")]
    file: String,
}

#[derive(Debug, Parser)]
pub struct CompileArgs {
    #[arg(help = "Input klang source code file")]
    file: String,
}
