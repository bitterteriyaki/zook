use crate::parser::NodeExit;

pub fn to_asm(root: NodeExit) -> String {
    let mut output = String::new();

    output.push_str("global _start\n");
    output.push_str("_start:\n");
    output.push_str("  mov rax, 60\n");
    output.push_str(&format!("  mov rdi, {}\n", root.expr.integer.value.unwrap()));
    output.push_str("  syscall\n");

    output
}
