use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use crate::ast::*;
use crate::ast::Ast::*;
use std::cmp::{max};
use std::iter::FromIterator;
use indexmap::map::IndexMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Func<'a> {
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
    context: IndexMap<&'a str, i32>,
    functions: HashMap<&'a str, Func<'a>>,
    result_name: Option<&'a str>
}

impl<'a> Env<'a> {
    pub fn new(initial: &IndexMap<&'a str, i32>) -> Self {
        Env {
            context: initial.clone(),
            functions: HashMap::new(),
            result_name: None
        }
    }

    pub fn from_parent(context: IndexMap<&'a str, i32>, functions: HashMap<&'a str, Func<'a>>) -> Self {
        Env {
            context,
            functions,
            result_name: None
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

    pub fn get_result(&mut self) -> Option<i32> {
        self.result_name.map(|n| self.get_or_create(n))
    }

    pub fn try_get(&self, var: &'a str) -> Option<&i32> {
        self.context.get(var)
    }

    pub fn set_result(&mut self, name: &'a str) {
        self.result_name = Some(name)
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
        Assign { var_name, fun_name, args } => {
            let func = env.get_function(fun_name).unwrap();
            let mut new_context = IndexMap::new();
            if args.len() != func.parameters.len() {
                panic!(format!("Function {} expected {} arguments, but got {}", fun_name, func.parameters.len(), args.len()));
            }

            for (index, param) in func.parameters.iter().enumerate() {
                let arg = &args[index];
                let value = match env.try_get(arg) {
                    Some(i) => *i,
                    None => 0
                };
                new_context.insert(param.as_ref(), value);
            }

            let mut new_env = Env::from_parent(new_context, env.functions.clone());
            interpret_program(&mut new_env, func.body);
            match new_env.get_result() {
                None => panic!(format!("Function {} returned no result", fun_name)),
                Some(i) => env.set(var_name, i)
            }
        }
        Return { name } => {
            env.set_result(name)
        }
    }
    println!("{}", env);
    println!();
}

pub fn interpret_program<'a>(env: &mut Env<'a>, program: &'a Vec<Ast>) {
    for expr in program {
        interpret(env, expr)
    }
}