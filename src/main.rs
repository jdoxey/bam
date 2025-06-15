use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod codegen;

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

fn compile_program(statements: &[Stmt]) -> Vec<u8> {
    let codegen = codegen::CodeGenerator::new();
    codegen.compile_program(statements)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename.bam>", args[0]);
        process::exit(1);
    }
    
    let input_file = &args[1];
    let input_path = Path::new(input_file);
    
    if !input_path.exists() {
        eprintln!("Error: File '{}' not found", input_file);
        process::exit(1);
    }
    
    if !input_file.ends_with(".bam") {
        eprintln!("Error: File must have .bam extension");
        process::exit(1);
    }
    
    // Read the input file
    let source_code = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file, e);
            process::exit(1);
        }
    };
    
    let parser = grammar::StmtParser::new();
    
    // Parse the statements line by line
    let mut statements = Vec::new();
    for (line_num, line) in source_code.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        match parser.parse(line) {
            Ok(stmt) => statements.push(stmt),
            Err(e) => {
                eprintln!("Parse error on line {}: {:?}", line_num + 1, e);
                process::exit(1);
            }
        }
    }
    
    if statements.is_empty() {
        eprintln!("Error: No valid statements found in '{}'", input_file);
        process::exit(1);
    }
    
    println!("Compiling {} with {} statements...", input_file, statements.len());
    
    // Compile to object code
    let object_bytes = compile_program(&statements);
    
    // Generate output filename (replace .bam with .o)
    let output_name = input_file.replace(".bam", "");
    let object_file = format!("{}.o", output_name);
    
    // Write object file
    match fs::write(&object_file, &object_bytes) {
        Ok(_) => println!("Generated object file: {} ({} bytes)", object_file, object_bytes.len()),
        Err(e) => {
            eprintln!("Error writing object file '{}': {}", object_file, e);
            process::exit(1);
        }
    }
}