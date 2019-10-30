mod ast;
mod parser;
mod interpreter;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    let filename = &args[1];
    let code_bytes = fs::read(filename).unwrap();
    let code: Vec<char> = String::from_utf8(code_bytes).unwrap().chars().collect();
    let program_parser = parser::program();
    match program_parser.parse(&code) {
        Ok(ast) => {
            let env = std::collections::HashMap::new();
            interpreter::interpret_program(env, ast);
        },
        Err(e) => println!("{}", e)
    }
}