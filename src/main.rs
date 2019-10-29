use pom::parser::*;
use crate::Ast::*;
use pom::set::Set;

#[derive(Debug, Eq, PartialEq)]
enum Ast<'a> {
    Def { name: &'a str, parameters: Vec<&'a str>, body: Vec<Ast<'a>> },
    Return { name: &'a str },
    While { cond_var: &'a str, body: Vec<Ast<'a>> },
    Assign { var_name: &'a str, fun_name: &'a str, args: Vec<&'a str> },
    Incr { var_name: &'a str },
    Decr { var_name: &'a str }
}

#[derive(Debug, Eq, PartialEq)]
struct Program<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
    body: Vec<Ast<'a>>
}

fn separator<'a>() -> Parser<'a, char, ()> {
    (sym(';') | sym('\n')).discard()
}

fn var_name<'a>() -> Parser<'a, char, &'a str> {
    is_a(|c: char| c.is_alphanumeric()).repeat(0..).collect().map(|s| s.to_str())
}

fn return_expr<'a>() -> Parser<'a, char, Ast<'a>> {
    (tag("return ") * var_name()).map(|name| Return { name })
}

fn incr<'a>() -> Parser<'a, char, Ast<'a>> {
    (var_name() - tag("+=1")).map(|var_name| Incr { var_name: &var_name })
}

fn decr<'a>() -> Parser<'a, char, Ast<'a>> {
    (var_name() - tag("-=1")).map(|var_name| Decr { var_name: &var_name })
}

fn main() {
    println!("Hello, world!");
}
