use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use crate::ast::*;
use crate::ast::Ast::*;
use std::cmp::{max};

struct Func<'a> {
    body: &'a Vec<Ast>,
    parameters: &'a Vec<String>
}

impl<'a> Func<'a> {
    pub fn new(body: &'a Vec<Ast>, parameters: &'a Vec<String>) -> Self {
        Func {
            body,
            parameters
        }
    }
}

pub struct Env<'a> {
    context: HashMap<&'a str, i32>,
    functions: HashMap<&'a str, Func<'a>>
}

impl<'a> Env<'a> {
    pub fn new(initial: &HashMap<&'a str, i32>) -> Self {
        Env {
            context: initial.clone(),
            functions: HashMap::new()
        }
    }

    pub fn get_or_create(&mut self, var: &'a str) -> i32 {
        match self.context.get(var) {
            None => {
                self.context.insert(var, 0);
                0
            },
            Some(val) => *val
        }
    }

    pub fn set(&mut self, var: &'a str, value: i32) {
        self.context.insert(var, value);
    }

    pub fn set_function(&mut self, name: &'a str, func: Func<'a>) {
        self.functions.insert(name, func);
    }

    pub fn get_function(&self, name: &'a str) -> Result<&Func, String> {
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

fn interpret<'a>(env: &mut Env<'a>, expr: &'a Ast) {
    println!("Executing: {}", expr);
    println!("{}", env);
    println!("===>");
    match expr {
        Incr { var_name } => {
            let old = env.get_or_create(var_name);
            env.set(var_name, old + 1)
        }
        Decr { var_name } => {
            let old = env.get_or_create(var_name);
            let new = max(0, old - 1);
            env.set(var_name, new)
        }
        While { cond_var, body } => {
            loop {
                if env.get_or_create(cond_var) == 0 {
                    break;
                } else {
                    interpret_program(env, body)
                }
            }
        }
        Def { name, body, parameters } => {
            let func = Func::new(body, parameters);
            env.set_function(name, func)
        }
        _ => unimplemented!()
    }
    println!("{}", env);
    println!();
}

pub fn interpret_program<'a>(env: &mut Env<'a>, program: &'a Vec<Ast>) {
    for expr in program {
        interpret(env, expr)
    }
}