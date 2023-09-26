use crate::parser::*;

/// Responsible for converting a parsed syntax tree into x86-64 assembly language.
pub struct Generator {
    root: NodeExit,
}

impl Generator {
    /// Constructs a new `Generator` from a provided root node.
    ///
    /// # Parameters
    ///
    /// * `root`: The root node of the syntax tree, representing an exit statement.
    ///
    /// # Returns
    ///
    /// A new `Generator` instance.
    pub fn new(root: NodeExit) -> Self {
        Self { root }
    }

    /// Generates the assembly code from the provided syntax tree.
    ///
    /// The generated code will invoke a system exit call with a status code from the root node.
    ///
    /// # Returns
    ///
    /// A string containing the generated x86-64 assembly code.
    pub fn generate(&self) -> String {
        format!(
            "global _start\n_start:\n    mov rax, 60\n    mov rdi, {} \n    syscall\n",
            self.root.expr.int_value
        )
    }
}
