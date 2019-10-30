use crate::ast::*;
use crate::ast::Ast::*;

type Env<'a> = std::collections::HashMap<&'a str, u32>;

fn interpret<'a>(env: &mut Env<'a>, expr: Ast) {
    env.insert("test", 1);
}

pub fn interpret_program<'a>(mut env: Env<'a>, program: Vec<Ast>) {
    for expr in program {
        interpret(&mut env, expr)
    }
}