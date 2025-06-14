use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
}

lalrpop_mod!(pub grammar);

fn eval_expr(expr: &Expr, env: &HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(l, r) => eval_expr(l, env) + eval_expr(r, env),
        Expr::Var(name) => *env.get(name).unwrap_or(&0),
    }
}

fn main() {
    let mut env = HashMap::new();
    let parser = grammar::StmtParser::new();
    
    let assign_stmt = "x = 5";
    let ast: Stmt = parser.parse(assign_stmt).unwrap();
    println!("Parsed: {:#?}", ast);
    
    match ast {
        Stmt::Assign(var, expr) => {
            let value = eval_expr(&expr, &env);
            env.insert(var.clone(), value);
            println!("Assigned {} = {}", var, value);
        }
        Stmt::Expr(expr) => {
            let value = eval_expr(&expr, &env);
            println!("Result: {}", value);
        }
    }
    
    let use_var = "x";
    let expr_ast: Expr = grammar::ExprParser::new().parse(use_var).unwrap();
    let result = eval_expr(&expr_ast, &env);
    println!("Variable x = {}", result);
}
