use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
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

fn link_executable(object_file: &str, executable_name: &str) -> Result<(), String> {
    // Use hybrid LLD approach for all platforms
    let lld_path = get_lld_for_linking()?;
    link_with_lld(&lld_path, object_file, executable_name)
}

fn get_lld_for_linking() -> Result<PathBuf, String> {
    // 1. Try bundled LLD first (for packaged installations)
    match get_bundled_lld() {
        Ok(lld_path) => {
            println!("Using bundled LLD: {}", lld_path.display());
            return Ok(lld_path);
        }
        Err(_) => {
            // Continue to development fallback
        }
    }
    
    // 2. For development: try rust-lld from toolchain
    #[cfg(debug_assertions)]
    {
        match get_rust_lld() {
            Ok(lld_path) => {
                println!("Development mode: Using rust-lld from toolchain: {}", lld_path.display());
                return Ok(lld_path);
            }
            Err(_) => {
                // Continue to error
            }
        }
    }
    
    // 3. No LLD found - provide helpful error message
    Err(format!(
        "No LLD linker found.\n\
        \n\
        For development: Ensure Rust is properly installed via rustup.\n\
        For distribution: Re-download the complete bam package from:\n\
        https://github.com/jdoxey/bam/releases\n\
        \n\
        Searched for:\n\
        - Bundled rust-lld at: {}\n\
        - Rust toolchain rust-lld", 
        get_bundled_lld_path().display()
    ))
}

fn get_bundled_lld_path() -> PathBuf {
    // Get expected path for bundled rust-lld (may not exist)
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            return exe_dir.join("rust-lld");
        }
    }
    PathBuf::from("rust-lld") // fallback
}

fn get_bundled_lld() -> Result<PathBuf, String> {
    // Look for rust-lld in same directory as bam executable
    let lld_path = get_bundled_lld_path();
    
    if lld_path.exists() {
        Ok(lld_path)
    } else {
        Err(format!(
            "Bundled LLD linker not found at: {}\n\
            This indicates a corrupted or incomplete bam installation.\n\
            Please re-download the complete bam distribution package from:\n\
            https://github.com/jdoxey/bam/releases",
            lld_path.display()
        ))
    }
}

fn get_rust_lld() -> Result<PathBuf, String> {
    // Try to find rust-lld from the current Rust toolchain
    let output = process::Command::new("rustc")
        .arg("--print")
        .arg("target-libdir")
        .output()
        .map_err(|e| format!("Failed to run rustc: {}", e))?;

    if !output.status.success() {
        return Err("rustc command failed".to_string());
    }

    let target_libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let rust_lld = Path::new(&target_libdir).join("../bin/rust-lld");
    
    if rust_lld.exists() {
        Ok(rust_lld)
    } else {
        Err(format!("rust-lld not found at: {}", rust_lld.display()))
    }
}

fn link_with_lld(lld_path: &Path, object_file: &str, executable_name: &str) -> Result<(), String> {
    let mut cmd = process::Command::new(lld_path);
    
    // Use platform-specific linking arguments based on the host platform
    #[cfg(target_os = "linux")]
    {
        let (lib_dir, linker_path) = if cfg!(target_arch = "x86_64") {
            ("x86_64-linux-gnu", "/lib64/ld-linux-x86-64.so.2")
        } else if cfg!(target_arch = "aarch64") {
            ("aarch64-linux-gnu", "/lib/ld-linux-aarch64.so.1")
        } else {
            panic!("Unsupported Linux architecture: {}", std::env::consts::ARCH);
        };
        
        cmd.arg("-flavor")
            .arg("gnu")  // Use GNU ld-compatible interface
            .arg("-o")
            .arg(executable_name)
            .arg(&format!("/usr/lib/{}/crt1.o", lib_dir))  // C runtime startup
            .arg(&format!("/usr/lib/{}/crti.o", lib_dir))  // C runtime init
            .arg(object_file)                              // Our object file
            .arg(&format!("/usr/lib/{}/crtn.o", lib_dir))  // C runtime finish
            .arg("-lc")  // Link against libc
            .arg(&format!("-L/usr/lib/{}", lib_dir))  // Add library search path
            .arg(&format!("-L/lib/{}", lib_dir))      // Add another library search path
            .arg("-L/lib64")                          // Add lib64 path
            .arg("-dynamic-linker")
            .arg(linker_path);  // Set dynamic linker path
    }
    
    #[cfg(target_os = "macos")]
    {
        // Architecture is mandatory for darwin flavor
        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"  // Try aarch64 instead of arm64
        } else {
            panic!("Unsupported macOS architecture: {}", std::env::consts::ARCH);
        };
        
        cmd.arg("-flavor")
            .arg("darwin")
            .arg("-arch")
            .arg(arch)
            .arg("-o")
            .arg(executable_name)
            .arg(object_file)                    // Our object file
            .arg("-lSystem");                    // Link against libSystem (includes libc)
    }
    
    #[cfg(target_os = "windows")]
    {
        cmd.arg("-flavor")
            .arg("link")  // Use MSVC linker interface
            .arg(&format!("/out:{}", executable_name))
            .arg(object_file)                    // Our object file
            .arg("/defaultlib:msvcrt")           // Link against MSVC runtime
            .arg("/defaultlib:kernel32")         // Link against kernel32
            .arg("/subsystem:console");          // Console application
    }
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute LLD: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("LLD linking failed: {}", stderr));
    }

    Ok(())
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

    // Link to create executable
    let executable_name = &output_name;
    match link_executable(&object_file, executable_name) {
        Ok(_) => {
            println!("Generated executable: {}", executable_name);
            // Clean up object file
            let _ = fs::remove_file(&object_file);
        }
        Err(e) => {
            eprintln!("Error linking executable '{}': {}", executable_name, e);
            process::exit(1);
        }
    }
}