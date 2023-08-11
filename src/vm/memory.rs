pub struct StackFrame {
    locals: Vec<i32>,
    args: Vec<i32>,
    ip: usize,
}
