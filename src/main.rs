use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Var(String),
    Str(String),
    Call(String, Vec<(String, Expr)>),
}

#[derive(Debug)]
pub enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
}

lalrpop_mod!(pub grammar);

#[derive(Debug, Clone)]
pub enum Value {
    Num(i32),
    Str(String),
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
        Expr::Call(func_name, args) => {
            if func_name == "print" {
                if let Some((param_name, expr)) = args.first() {
                    if param_name == "message" {
                        let value = eval_expr(expr, env);
                        match value {
                            Value::Str(s) => println!("{}", s),
                            Value::Num(n) => println!("{}", n),
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

fn main() {
    let mut env = HashMap::new();
    let parser = grammar::StmtParser::new();
    
    let assign_stmt = "x = 5";
    let ast: Stmt = parser.parse(assign_stmt).unwrap();
    println!("Parsed: {:#?}", ast);
    
    match ast {
        Stmt::Assign(var, expr) => {
            let value = eval_expr(&expr, &env);
            match &value {
                Value::Num(n) => println!("Assigned {} = {}", var, n),
                Value::Str(s) => println!("Assigned {} = \"{}\"", var, s),
            }
            env.insert(var.clone(), value);
        }
        Stmt::Expr(expr) => {
            let value = eval_expr(&expr, &env);
            match value {
                Value::Num(n) => println!("Result: {}", n),
                Value::Str(s) => println!("Result: \"{}\"", s),
            }
        }
    }
    
    let print_stmt = r#"print(message: "Hello, world!")"#;
    let print_ast: Stmt = parser.parse(print_stmt).unwrap();
    println!("Parsed print: {:#?}", print_ast);
    
    match print_ast {
        Stmt::Expr(expr) => {
            eval_expr(&expr, &env);
        }
        _ => {}
    }
    
    let print_var = r#"print(message: x)"#;
    let print_var_ast: Stmt = parser.parse(print_var).unwrap();
    
    match print_var_ast {
        Stmt::Expr(expr) => {
            eval_expr(&expr, &env);
        }
        _ => {}
    }
}
