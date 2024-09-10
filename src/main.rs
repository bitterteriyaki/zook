mod tokens;
mod parser;
mod generator;

use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use tokens::tokenize;
use parser::Parser;
use generator::to_asm;

const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn main() {
    use std::panic;

    panic::set_hook(Box::new(|info| {
        eprintln!("{}[zook fatal error]{}", RED, RESET);

        if let Some(message) = info.payload().downcast_ref::<&str>() {
            eprintln!("{}{message}{}", BOLD, RESET);
        } else if let Some(message) = info.payload().downcast_ref::<String>() {
            eprintln!("{}{message}{}", BOLD, RESET);
        } else {
            eprintln!("{}An unknown error occurred{}", BOLD, RESET);
        }
    }));

    let args: Vec<String> = args().collect();
    let executable = &args[0];

    if args.len() < 2 {
        panic!("Usage: {} <FILE>", executable);
    }

    let mut file = File::open(&args[1]).expect("Error: File not found");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Error: Could not read file");

    let tokens = tokenize(&content);

    let parser = Parser::new(tokens);
    let root = parser.get_root();

    let asm = to_asm(root);

    let mut file = File::create("out.asm")
        .expect("Error: Could not create file");

    file.write_all(asm.as_bytes()).expect("Error: Could not write to file");

    Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("out.asm")
        .output()
        .expect("Error: Could not assemble file");

    Command::new("ld")
        .arg("out.o")
        .arg("-o")
        .arg("out")
        .output()
        .expect("Error: Could not link file");
}
