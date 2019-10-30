use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use crate::ast::*;
use crate::ast::Ast::*;

pub struct Env<'a> {
    context: HashMap<&'a str, i32>,
    functions: HashMap<&'a str, Ast>
}

impl<'a> Env<'a> {
    pub fn new(initial: &HashMap<&'a str, i32>) -> Self {
        Env {
            context: initial.clone(),
            functions: HashMap::new()
        }
    }

    pub fn get_or_create(&mut self, var: &'a str, value: i32) -> i32 {
        match self.context.get(var) {
            None => {
                self.context.insert(var, value);
                0
            },
            Some(val) => *val
        }
    }

    pub fn set(&mut self, var: &'a str, value: i32) {
        self.context.insert(var, value);
    }

    pub fn set_function(&mut self, name: &'a str, expr: Ast) {
        self.functions.insert(name, expr);
    }

    pub fn get_function(&self, name: &'a str) -> Result<&Ast, String> {
        match self.functions.get(name) {
            None => Err(format!("Function {} does not exist", name)),
            Some(fun) => Ok(fun)
        }
    }
}

impl<'a> Display for Env<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let entries = self.context.iter().map(|(key, value)| format!("{}: {}", key, value)).collect::<Vec<String>>();
        write!(f, "[{}]", entries.join(", "))
    }
}

fn interpret<'a>(env: &mut Env<'a>, expr: &Ast) {
    println!("{}", env);
    println!("Executing: {}", expr);
    match expr {
        _ => ()
    }
    println!("{}", env);
}

pub fn interpret_program<'a>(mut env: Env<'a>, program: Vec<Ast>) {
    for expr in program {
        interpret(&mut env, &expr)
    }
}