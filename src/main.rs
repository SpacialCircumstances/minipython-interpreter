mod ast;
mod parser;
mod interpreter;

use std::env;
use std::fs;
use indexmap::map::IndexMap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("minipython")
        .version("0.1.0")
        .about("A interpreter for minipython")
        .author("SpacialCircumstances")
        .arg(Arg::with_name("interactive")
            .help("Run in interactive mode, allowing you to specify input and output in the script")
            .short("i")
            .long("interactive")
        )
        .arg(Arg::with_name("file")
            .help("The script file to run")
            .required(true)
            .index(1)
        ).get_matches();
    if let Some(filename) = matches.value_of("file") {
        let code_bytes = fs::read(filename).unwrap();
        let code: Vec<char> = String::from_utf8(code_bytes).unwrap().chars().collect();
        let program_parser = parser::program();
        match program_parser.parse(&code) {
            Ok(ast) => {
                let env_initial = IndexMap::new();
                let mut env = interpreter::Env::new(&env_initial);
                interpreter::interpret_program(&mut env, &ast);
            },
            Err(e) => println!("{}", e)
        }
    }
}