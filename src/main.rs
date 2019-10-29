use pom::parser::*;
use pom::char_class::alphanum;
use crate::Ast::*;

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

fn var_name() -> Parser<u8, &str> {
    is_a(alphanum).repeat(0..).map(|chars| chars.concat())
}

fn return_expr() -> Parser<u8, Ast> {
    (seq(b"return ") * var_name()).map(|name| Return { name })
}

fn incr() -> Parser<u8, Ast> {
    (var_name() - seq(b"+=1".as_ref())).map(|var_name| Incr { var_name })
}

fn decr() -> Parser<u8, Ast> {
    (var_name() - seq(b"-=1".as_ref())).map(|var_name| Decr { var_name })
}

fn main() {
    println!("Hello, world!");
}
