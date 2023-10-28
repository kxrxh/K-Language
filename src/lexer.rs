use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct PestGrammarParser;

pub(crate) fn parse_code(code: &str) -> Option<pest::iterators::Pairs<'_, Rule>> {
    match PestGrammarParser::parse(Rule::program, code) {
        Ok(pairs) => Some(pairs),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}
