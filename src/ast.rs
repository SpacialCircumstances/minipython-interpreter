use std::fmt::{Display, Formatter, Error};
use crate::ast::Ast::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Ast {
    Def { name: String, parameters: Vec<String>, body: Vec<Ast> },
    Return { name: String },
    While { cond_var: String, body: Vec<Ast> },
    Assign { var_name: String, fun_name: String, args: Vec<String> },
    Incr { var_name: String },
    Decr { var_name: String }
}

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Decr { var_name } => write!(f, "{}-=1", var_name),
            Incr { var_name } => write!(f, "{}+=1", var_name),
            Return { name } => write!(f, "return {}", name),
            Assign { var_name, fun_name, args } => write!(f, "{}={}({})", var_name, fun_name, args.join(", ")),
            While { cond_var, body } => {
                let body_strs: Vec<String> = body.iter().map(|expr| format!("{}", expr)).collect();
                write!(f, "while {}!=0: {}", cond_var, body_strs.join("; "))
            },
            Def { name, parameters, body } => {
                let body_strs: Vec<String> = body.iter().map(|expr| format!("{}", expr)).collect();
                write!(f, "def {}({}): {}", name, parameters.join(", "), body_strs.join("; "))
            }
        }
    }
}