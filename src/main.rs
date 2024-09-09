mod tokens;
mod parser;
mod generator;

use tokens::Tokenizer;
use generator::to_asm;
use parser::Parser;
use std::env::args;
use std::io::{Read, Result, Write};
use std::process::Command;
use std::fs::File;


fn main() -> Result<()> {
    // Maybe parse the command line with clap or structopt and use colored crate
    // to colorize the output properly.
    let args: Vec<String> = args().collect();
    let executable = &args[0];

    if args.len() < 2 {
        panic!("Usage: {} <file>\n", executable);
    }

    // Show the correct error message if the file does not exist using the
    // `fatal` function created above.
    let mut file = File::open(&args[1])?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let tokenizer = Tokenizer::new(&content);
    let tokens = tokenizer.tokenize();
    
    let parser = Parser::new(tokens);
    let root = parser.parse();

    let asm = to_asm(root);
     
    let mut file = File::create("out.asm")?;
    file.write_all(asm.as_bytes())?;

    Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("out.asm")
        .output()?;

    Command::new("ld")
        .arg("out.o")
        .arg("-o")
        .arg("out")
        .output()?;

    Ok(())
}
