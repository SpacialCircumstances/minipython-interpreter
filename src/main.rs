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

fn main() {
    println!("Hello, world!");
}
