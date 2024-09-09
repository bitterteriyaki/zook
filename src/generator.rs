use crate::parser::NodeExit;

pub fn to_asm(root: NodeExit) -> String {
    let mut asm = String::new();

    asm.push_str("global _start\n");
    asm.push_str("_start:\n");
    asm.push_str("  mov rax, 60\n");
    asm.push_str(&format!("  mov rdi, {}\n", root.expr.integer.value.unwrap()));
    asm.push_str("  syscall\n");

    asm
}
