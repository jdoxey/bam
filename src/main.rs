use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
    Str(String),
    Call(String, Vec<(String, Expr)>),
    Eq(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
    If(Expr, Vec<Stmt>),
}

lalrpop_mod!(pub grammar);

#[derive(Debug, Clone)]
pub enum Value {
    Num(i32),
    Str(String),
    Bool(bool),
}

fn eval_expr(expr: &Expr, env: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Str(s) => Value::Str(s.clone()),
        Expr::Add(l, r) => {
            let left = eval_expr(l, env);
            let right = eval_expr(r, env);
            match (left, right) {
                (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
                _ => panic!("Cannot add non-numbers"),
            }
        }
        Expr::Var(name) => env.get(name).cloned().unwrap_or(Value::Num(0)),
        Expr::Eq(l, r) => {
            let left = eval_expr(l, env);
            let right = eval_expr(r, env);
            match (left, right) {
                (Value::Num(a), Value::Num(b)) => Value::Bool(a == b),
                (Value::Str(a), Value::Str(b)) => Value::Bool(a == b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                _ => Value::Bool(false),
            }
        }
        Expr::Call(func_name, args) => {
            if func_name == "print" {
                if let Some((param_name, expr)) = args.first() {
                    if param_name == "message" {
                        let value = eval_expr(expr, env);
                        match value {
                            Value::Str(s) => println!("{}", s),
                            Value::Num(n) => println!("{}", n),
                            Value::Bool(b) => println!("{}", b),
                        }
                        Value::Num(0)
                    } else {
                        panic!("print() requires 'message' parameter");
                    }
                } else {
                    panic!("print() requires a message parameter");
                }
            } else {
                panic!("Unknown function: {}", func_name);
            }
        }
    }
}

fn eval_stmt(stmt: &Stmt, env: &mut HashMap<String, Value>) {
    match stmt {
        Stmt::Assign(var, expr) => {
            let value = eval_expr(expr, env);
            env.insert(var.clone(), value);
        }
        Stmt::Expr(expr) => {
            eval_expr(expr, env);
        }
        Stmt::If(cond, body) => {
            let condition = eval_expr(cond, env);
            let is_true = match condition {
                Value::Bool(b) => b,
                Value::Num(n) => n != 0,
                Value::Str(s) => !s.is_empty(),
            };
            if is_true {
                for stmt in body {
                    eval_stmt(stmt, env);
                }
            }
        }
    }
}

fn main() {
    let mut env = HashMap::new();
    let parser = grammar::StmtParser::new();
    
    // Test assignment
    let assign_stmt = "x = 5";
    let ast: Stmt = parser.parse(assign_stmt).unwrap();
    println!("Executing: {}", assign_stmt);
    eval_stmt(&ast, &mut env);
    
    // Test simple if statement with equality
    let if_stmt = r#"if x == 5 {
        print(message: "x equals 5!")
    }"#;
    
    println!("Executing: {}", if_stmt);
    match parser.parse(if_stmt) {
        Ok(if_ast) => eval_stmt(&if_ast, &mut env),
        Err(e) => println!("Parse error: {:?}", e),
    }
}