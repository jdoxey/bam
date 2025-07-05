use lalrpop_util::lalrpop_mod;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

// Removed unused import std::os::unix; it was unused even on macOS
// because std::os::unix::fs::symlink is called with its full path.

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

// Removed unused function eval_expr
// Removed unused function eval_stmt

fn compile_program(statements: &[Stmt]) -> Vec<u8> {
    let codegen = codegen::CodeGenerator::new();
    codegen.compile_program(statements)
}

fn link_executable(object_file: &str, executable_name: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // On Windows, we must use the bundled clang driver which is packaged with the
        // necessary MinGW libraries and configuration.
        match get_bundled_clang() {
            Ok(clang_path) => {
                println!("Using bundled clang for linking: {}", clang_path.display());
                link_with_clang(&clang_path, object_file, executable_name)
            }
            Err(e) => Err(format!(
                "{e}.\nBam on Windows requires the bundled clang.exe for linking.\n\
                Please ensure you are running a packaged version of bam from the releases page, or\n\
                if developing, ensure clang.exe and the required MinGW libraries are in the correct location."
            )),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // For other OSes (Linux/macOS), find LLD and link directly.
        let lld_path = get_lld_for_linking()?;
        link_with_lld(&lld_path, object_file, executable_name)
    }
}

#[cfg(not(target_os = "windows"))]
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
    match get_rust_lld() {
        Ok(lld_path) => {
            println!("Using rust-lld from toolchain: {}", lld_path.display());
            return Ok(lld_path);
        }
        Err(_) => {
            // Continue to error
        }
    }

    // 3. No LLD found - provide helpful error message
    Err(format!(
        "No LLD linker found.\n\
        \n
        For development: Ensure Rust is properly installed via rustup.\n\
        For distribution: Re-download the complete bam package from:\n\
        https://github.com/jdoxey/bam/releases\n\
        \n\
        Searched for:\n\
        - Bundled linker (e.g., ld.lld on Linux, lld on macOS, lld.exe on Windows) at: {}\n\
        - Rust toolchain rust-lld",
        get_bundled_lld_path().display()
    ))
}

#[cfg(not(target_os = "windows"))]
fn get_bundled_lld_path() -> PathBuf {
    // Get expected path for bundled rust-lld (may not exist)
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if cfg!(target_os = "windows") {
                return exe_dir.join("ld.lld.exe");
            } else if cfg!(target_os = "linux") {
                return exe_dir.join("ld.lld"); // Specific for Linux
            } else {
                // Other Unix (macOS)
                return exe_dir.join("lld");
            }
        }
    }
    // fallback
    if cfg!(target_os = "windows") {
        PathBuf::from("ld.lld.exe")
    } else if cfg!(target_os = "linux") {
        // Specific for Linux
        PathBuf::from("ld.lld")
    } else {
        // Other Unix (macOS)
        PathBuf::from("lld")
    }
}

#[cfg(not(target_os = "windows"))]
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

#[cfg(not(target_os = "windows"))]
fn get_rust_lld() -> Result<PathBuf, String> {
    // Try to find rust-lld from the current Rust toolchain
    let output = process::Command::new("rustc")
        .arg("--print")
        .arg("target-libdir")
        .output()
        .map_err(|e| format!("Failed to run rustc: {e}"))?;

    if !output.status.success() {
        return Err("rustc command failed".to_string());
    }

    let target_libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let rust_lld = Path::new(&target_libdir).join("../bin/rust-lld");

    if rust_lld.exists() {
        return Ok(rust_lld);
    }
    Err(format!("rust-lld not found at: {}", rust_lld.display()))
}

/// Try to find bundled clang.exe for Windows linking, next to the bam executable.
#[cfg(target_os = "windows")]
#[allow(dead_code)]
fn get_bundled_clang() -> Result<PathBuf, String> {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let clang = exe_dir.join("clang.exe");
            if clang.exists() {
                return Ok(clang);
            }
        }
    }
    Err("Bundled clang.exe not found".to_string())
}

/// Link using the bundled clang driver on Windows, passing through to LLD for COFF support.
#[cfg(target_os = "windows")]
fn link_with_clang(
    clang_path: &Path,
    object_file: &str,
    executable_name: &str,
) -> Result<(), String> {
    let mut cmd = process::Command::new(clang_path);

    // Determine the lib path relative to clang executable
    let clang_dir = clang_path.parent().ok_or("Cannot get clang directory")?;
    let lib_path = clang_dir.join("lib");

    if !lib_path.exists() {
        return Err(format!("Library path not found: {}", lib_path.display()));
    }

    // Debug: list contents of lib directory
    eprintln!("Contents of lib directory:");
    if let Ok(entries) = std::fs::read_dir(&lib_path) {
        for entry in entries.flatten() {
            eprintln!("  {}", entry.file_name().to_string_lossy());
        }
    }

    // Check if ld.lld.exe exists next to clang.exe
    let lld_path = clang_dir.join("ld.lld.exe");
    if lld_path.exists() {
        eprintln!("Found LLD at: {}", lld_path.display());
    } else {
        eprintln!("LLD not found at: {}", lld_path.display());
    }

    // Use correct target triple for MinGW UCRT
    // Let clang automatically handle startup objects and library linking
    cmd.arg("-target")
        .arg("x86_64-w64-mingw32")
        .arg("-fuse-ld=lld")
        .arg("-v") // Add verbose output for debugging
        .arg(format!("-L{}", lib_path.display())) // Library search path
        .arg(object_file) // Our object file
        .arg("-o")
        .arg(executable_name)
        .arg("-lucrt") // Use UCRT instead of msvcrt
        .arg("-lkernel32")
        .arg("-Wl,-subsystem,console")
        // Explicitly tell clang where to find the linker if it exists
        .arg(format!("-B{}", clang_dir.display()));

    // Debug: print the command we're about to run
    eprintln!(
        "Clang command: {} {}",
        clang_path.display(),
        cmd.get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute clang for linking: {e}"))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    eprintln!("Clang exit status: {}", output.status);
    eprintln!("Clang STDOUT: {}", stdout);
    eprintln!("Clang STDERR: {}", stderr);

    if !output.status.success() {
        return Err(format!(
            "Clang linking failed with exit code {}:\nSTDOUT: {stdout}\nSTDERR: {stderr}",
            output.status.code().unwrap_or(-1)
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn link_with_lld(lld_path: &Path, object_file: &str, executable_name: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // Use ld64.lld symlink approach for macOS (discovered to work in debugging)
        let lld_dir = lld_path
            .parent()
            .ok_or("Cannot get parent directory of LLD")?;
        let ld64_path = lld_dir.join("ld64.lld");

        // Create ld64.lld symlink to rust-lld
        if !ld64_path.exists() {
            std::os::unix::fs::symlink(lld_path, &ld64_path)
                .map_err(|e| format!("Failed to create ld64.lld symlink: {e}"))?;
        }

        let mut cmd = process::Command::new(&ld64_path);

        // Architecture is mandatory for Darwin linker - use Apple's naming
        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "arm64" // Use Apple's standard ARM64 naming for Darwin
        } else {
            panic!("Unsupported macOS architecture: {}", std::env::consts::ARCH);
        };

        // Try to find SDK path dynamically
        let sdk_paths = [
            "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk",
            "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk",
            "/usr/lib", // Fallback to standard lib directory
        ];

        let mut sdk_found = false;
        for sdk_path in &sdk_paths {
            if std::path::Path::new(sdk_path).exists() {
                if sdk_path.ends_with(".sdk") {
                    cmd.arg("-syslibroot").arg(sdk_path);
                } else {
                    cmd.arg("-L").arg(sdk_path);
                }
                sdk_found = true;
                break;
            }
        }

        if !sdk_found {
            return Err("No macOS SDK found. Please install Xcode Command Line Tools: xcode-select --install".to_string());
        }

        cmd.arg("-arch")
            .arg(arch)
            .arg("-platform_version")
            .arg("macos")
            .arg("11.0") // Minimum macOS version
            .arg("14.0") // SDK version
            .arg("-o")
            .arg(executable_name)
            .arg(object_file) // Our object file
            .arg("-lSystem"); // Link against libSystem (includes libc)

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute ld64.lld: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "ld64.lld linking failed: {stderr}

                On macOS, bam requires Xcode Command Line Tools to be installed.

                Install them with: xcode-select --install

                

                Alternatively, you can install LLVM via Homebrew: brew install llvm",
            ));
        }
    }

    #[cfg(target_os = "linux")]
    {
        let mut cmd = process::Command::new(lld_path);

        let (lib_dir, linker_path) = if cfg!(target_arch = "x86_64") {
            ("x86_64-linux-gnu", "/lib64/ld-linux-x86-64.so.2")
        } else if cfg!(target_arch = "aarch64") {
            ("aarch64-linux-gnu", "/lib/ld-linux-aarch64.so.1")
        } else {
            panic!("Unsupported Linux architecture: {}", std::env::consts::ARCH);
        };

        cmd.arg("-flavor")
            .arg("gnu") // Use GNU ld-compatible interface
            .arg("-o")
            .arg(executable_name)
            .arg(format!("/usr/lib/{lib_dir}/crt1.o")) // C runtime startup
            .arg(format!("/usr/lib/{lib_dir}/crti.o")) // C runtime init
            .arg(object_file) // Our object file
            .arg(format!("/usr/lib/{lib_dir}/crtn.o")) // C runtime finish
            .arg("-lc") // Link against libc
            .arg(format!("-L/usr/lib/{lib_dir}")) // Add library search path
            .arg(format!("-L/lib/{lib_dir}")) // Add another library search path
            .arg("-L/lib64") // Add lib64 path
            .arg("-dynamic-linker")
            .arg(linker_path); // Set dynamic linker path

        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute LLD: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("LLD linking failed: {stderr}"));
        }
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
        eprintln!("Error: File '{input_file}' not found");
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
            eprintln!("Error reading file '{input_file}': {e}");
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
        eprintln!("Error: No valid statements found in '{input_file}'");
        process::exit(1);
    }

    println!(
        "Compiling {} with {} statements...",
        input_file,
        statements.len()
    );

    // Compile to object code
    let object_bytes = compile_program(&statements);

    // Generate output filename (replace .bam with .o)
    let output_name = input_file.replace(".bam", "");
    let object_file = format!("{output_name}.o");

    // Write object file
    match fs::write(&object_file, &object_bytes) {
        Ok(_) => println!(
            "Generated object file: {} ({} bytes)",
            object_file,
            object_bytes.len()
        ),
        Err(e) => {
            eprintln!("Error writing object file '{object_file}': {e}");
            process::exit(1);
        }
    }

    // Link to create executable
    let executable_name = if cfg!(target_os = "windows") {
        format!("{output_name}.exe")
    } else {
        output_name.clone()
    };
    match link_executable(&object_file, &executable_name) {
        Ok(_) => {
            println!("Generated executable: {executable_name}");
            // Clean up object file (skip cleanup if KEEP_OBJECT_FILE env var is set)
            if env::var("KEEP_OBJECT_FILE").is_err() {
                let _ = fs::remove_file(&object_file);
            }
        }
        Err(e) => {
            eprintln!("Error linking executable '{executable_name}': {e}");
            process::exit(1);
        }
    }
}
