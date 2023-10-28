use crate::lexer::Rule;

pub(crate) fn run_parser(pairs: pest::iterators::Pairs<Rule>) {
    for pair in pairs {
        process_statement(pair.into_inner());
    }
}

fn process_statement(pairs: pest::iterators::Pairs<Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::VAR_DECLARATION => process_var_declaration(pair.into_inner()),
            Rule::FUNCTION_DECLARATION => process_function_declaration(pair.into_inner()),
            Rule::STATEMENT => process_statement(pair.into_inner()),
            _ => println!("rule: {:?}", pair.as_rule()),
        }
    }
}

fn process_function_declaration(pairs: pest::iterators::Pairs<Rule>) {
    let mut identifier = String::new();
    let mut args = String::new();
    let mut ret_type = "void".to_string();
    for pair in pairs {
        match pair.as_rule() {
            Rule::IDENTIFIER => identifier = pair.as_str().to_string(),
            Rule::FUNCTION_DECLARATION_ARGS_BLOCK => args = pair.as_str().to_string(),

        }
        println!("{:?}: {}", pair.as_rule(), pair.as_str());
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
            _ => println!("{}", pair.as_str().to_string()),
        }
    }

    println!("Variable: {}", identifier);
    println!("Value: {}", expression);
}
