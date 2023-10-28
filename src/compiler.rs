pub fn generate_var_declaration_on_stack(identifier: &str, value: &str) -> String {
    format!(
        "push {} ; Push value onto the stack\n",
        value
    )
}
