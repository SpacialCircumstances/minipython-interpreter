#[derive(Debug, Eq, PartialEq)]
pub enum Ast {
    Def { name: String, parameters: Vec<String>, body: Vec<Ast> },
    Return { name: String },
    While { cond_var: String, body: Vec<Ast> },
    Assign { var_name: String, fun_name: String, args: Vec<String> },
    Incr { var_name: String },
    Decr { var_name: String }
}