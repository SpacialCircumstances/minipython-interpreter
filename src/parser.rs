use pom::parser::*;
use crate::ast::*;
use crate::ast::Ast::*;
use std::iter::FromIterator;

fn comment<'a>() -> Parser<'a, char, ()> {
    let comment_text = not_a(|c| c == '\r' || c == '\n' || c == ';').repeat(0..).convert(|com| {
        let ct: String = com.into_iter().collect();
        match &ct[..] {
            "endwhile" => Err(""),
            "enddef" => Err(""),
            _ => Ok(ct)
        }
    });
    (sym('#') - comment_text).discard()
}

fn spaces<'a>() -> Parser<'a, char, ()> {
    (sym(' ').discard() | sym('\t').discard() | sym('\r').discard() | comment()).repeat(0..).discard()
}

fn whitespace<'a>() -> Parser<'a, char, ()> {
    (is_a(|c: char| c.is_whitespace()).discard() | comment()).repeat(0..).discard()
}

fn separator<'a>() -> Parser<'a, char, ()> {
    (sym(';') | sym('\n')).discard()
}

fn body<'a>() -> Parser<'a, char, Vec<Ast>> {
    list(call(expression), separator() - whitespace()) - separator().opt()
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
    sym('(') * list(whitespace() * var_name(), sym(',')) - sym(')')
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
    (head + body() - whitespace() - tag("#endwhile")).map(|(name, body)| While {
        cond_var: name,
        body
    })
}

fn def_expr<'a>() -> Parser<'a, char, Ast> {
    let head = tag("def ") * var_name() + args() - tag(":") - whitespace();
    (head + body() - whitespace() - tag("#enddef")).map(|((fname, fargs), body)| Def {
        name: fname,
        parameters: fargs,
        body
    })
}

fn expression<'a>() -> Parser<'a, char, Ast> {
    let expr = incr() | decr() | return_expr() | assign() | while_expr() | def_expr();
    spaces() * expr - spaces()
}

pub fn program<'a>() -> Parser<'a, char, Vec<Ast>> {
    body() - end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        let text: Vec<char> = "a+=1".chars().collect();
        let res = program().parse(&text).unwrap();
        let expected = vec![
            Incr { var_name: String::from("a") }
        ];
        assert_eq!(expected, res);
    }

    #[test]
    fn parse_2() {
        let text: Vec<char> = "while a!=0: a-=1; #endwhile".chars().collect();
        let res = program().parse(&text).unwrap();
        let expected = vec![
            While {
                cond_var: String::from("a"),
                body: vec![
                    Decr {
                        var_name: String::from("a")
                    }
                ]
            }
        ];
        assert_eq!(expected, res);
    }

    #[test]
    fn parse_3() {
        let text: Vec<char> = "x1+=1; def add(x, y): while x!=0: n+=1; x-=1 #endwhile; while y!=0: n+=1; y-=1 #endwhile; return n #enddef; a=add(x1, y1); a+=1".chars().collect();
        let res = program().parse(&text).unwrap();
        let expected = vec![
            Incr { var_name: String::from("x1") },
            Def {
                name: String::from("add"),
                parameters: vec![
                    String::from("x"),
                    String::from("y")
                ],
                body: vec![
                    While {
                        cond_var: String::from("x"),
                        body: vec![
                            Incr { var_name: String::from("n") },
                            Decr { var_name: String::from("x") }
                        ]
                    },
                    While {
                        cond_var: String::from("y"),
                        body: vec![
                            Incr { var_name: String::from("n") },
                            Decr { var_name: String::from("y") }
                        ]
                    },
                    Return { name: String::from("n") }
                ]
            },
            Assign {
                var_name: String::from("a"),
                fun_name: String::from("add"),
                args: vec![
                    String::from("x1"),
                    String::from("y1")
                ]
            },
            Incr { var_name: String::from("a") }
        ];
        assert_eq!(expected, res);
    }
}