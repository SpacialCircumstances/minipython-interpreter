use pom::parser::*;
use crate::Ast::*;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq)]
enum Ast {
    Def { name: String, parameters: Vec<String>, body: Vec<Ast> },
    Return { name: String },
    While { cond_var: String, body: Vec<Ast> },
    Assign { var_name: String, fun_name: String, args: Vec<String> },
    Incr { var_name: String },
    Decr { var_name: String }
}

fn whitespace<'a>() -> Parser<'a, char, ()> {
    is_a(|c: char| c.is_whitespace()).repeat(0..).discard()
}

fn separator<'a>() -> Parser<'a, char, ()> {
    (sym(';') | sym('\n')).discard()
}

fn var_name<'a>() -> Parser<'a, char, String> {
    is_a(|c: char| c.is_alphanumeric()).repeat(0..).collect().map(|s| String::from_iter(s.iter()))
}

fn return_expr<'a>() -> Parser<'a, char, Ast> {
    (tag("return ") * var_name()).map(|name| Return { name })
}

fn incr<'a>() -> Parser<'a, char, Ast> {
    (var_name() - tag("+=1")).map(|var_name| Incr { var_name })
}

fn decr<'a>() -> Parser<'a, char, Ast> {
    (var_name() - tag("-=1")).map(|var_name| Decr { var_name })
}

fn args<'a>() -> Parser<'a, char, Vec<String>> {
    sym('(') * list(var_name(), sym(',')) - sym(')')
}

fn assign<'a>() -> Parser<'a, char, Ast> {
    (var_name() - tag("=") + var_name() + args()).map(|((v, f), arguments)| Assign {
        var_name: v,
        fun_name: f,
        args: arguments
    })
}

fn while_expr<'a>() -> Parser<'a, char, Ast> {
    let head = tag("while ") * var_name() - tag("!=0:") - whitespace();
    let body = list(call(expression), separator());
    (head + body - tag("#endwhile")).map(|(name, body)| While {
        cond_var: name,
        body: body
    })
}

fn def_expr<'a>() -> Parser<'a, char, Ast> {
    let head = tag("def ") * var_name() + args() - tag(":");
    let body = list(call(expression), separator());
    (head + body - tag("#enddef")).map(|((fname, fargs), body)| Def {
        name: fname,
        parameters: fargs,
        body
    })
}

fn expression<'a>() -> Parser<'a, char, Ast> {
    incr() | decr() | return_expr() | assign() | while_expr() | def_expr()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        let text: Vec<char> = "a+=1".chars().collect();
        let res = expression().parse(&text).unwrap();
        let expected = Incr { var_name: String::from("a") };
        assert_eq!(expected, res);
    }
}