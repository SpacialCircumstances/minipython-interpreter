mod ast;
mod parser;
mod interpreter;

use std::fs;
use indexmap::map::IndexMap;
use clap::{App, Arg};
use std::io::stdin;

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
        let code_bytes = fs::read(filename).expect("Failed to read file");
        let code: Vec<char> = String::from_utf8(code_bytes).expect("Failed to decode file").chars().collect();
        if matches.is_present("interactive") {
            let program_parser = parser::interactive_program();
            match program_parser.parse(&code) {
                Ok(iprog) => {
                    let mut env_initial: IndexMap<&str, i32> = IndexMap::new();
                    for input in &iprog.inputs {
                        println!("{}=", input);
                        let mut str_value = String::new();
                        stdin().read_line(&mut str_value).expect("Failed to read line");
                        let value: i32 = str_value.trim().parse().expect("Failed to read number");
                        env_initial.insert(input, value);
                    }
                    let mut env = interpreter::Env::new(&env_initial);
                    env.set_result(&iprog.output);
                    interpreter::interpret_program(&mut env, &iprog.program);
                    println!("Result: {}={}", &iprog.output, env.get_result().unwrap());
                },
                Err(e) => panic!(format!("Parser error: {}. Make sure the script you are using is interactive!", e))
            }
        } else {
            let program_parser = parser::program();
            match program_parser.parse(&code) {
                Ok(ast) => {
                    let env_initial = IndexMap::new();
                    let mut env = interpreter::Env::new(&env_initial);
                    interpreter::interpret_program(&mut env, &ast);
                },
                Err(e) => panic!(format!("Parser error: {}. Make sure the script you are using is non-interactive!", e))
            }
        }
    }
}