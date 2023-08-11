use super::{instructions::Instructions, memory::StackFrame};
struct VirtualMachine {
    function_stack: Vec<StackFrame>,
    call_stack: Vec<StackFrame>,
    instructions: Vec<Instructions>,
}

impl VirtualMachine {
    /// Creates a new [`VirtualMachine`].
    pub fn new(instructions: Vec<Instructions>) -> Self {
        Self {
            function_stack: Vec::new(),
            call_stack: Vec::new(),
            instructions,
        }
    }
}