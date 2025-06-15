# Contributing to bam

## Prerequisites

**You must have Rust installed.** Install it from [rustup.rs](https://rustup.rs/)

That's it! The bam compiler automatically detects and uses the LLD linker from your Rust installation during development.

## Development Workflow

```bash
# Clone and build
git clone https://github.com/jdoxey/bam.git
cd bam
cargo build

# Test the compiler
echo 'print(message: "Hello, World!")' > hello.bam
./target/debug/bam hello.bam
./hello
```

## How It Works

The bam compiler uses a **hybrid linking approach**:

- **Development mode** (debug builds): Automatically uses `rust-lld` from your Rust toolchain
- **Distribution mode** (packaged releases): Uses bundled `ld.lld` for standalone operation

When you run `cargo build`, the compiler will show:
```
Development mode: Using rust-lld from toolchain: /path/to/rust-lld
```

This means everything is working correctly.

## Testing Changes

```bash
# Build and test
cargo build
./target/debug/bam hello.bam
./hello

# Run in release mode
cargo build --release
./target/release/bam hello.bam
./hello
```

## Architecture

- **Parser**: LALRPOP-based grammar in `src/grammar.lalrpop`
- **AST**: Expression and Statement types in `src/main.rs`
- **Codegen**: Cranelift-based code generation in `src/codegen.rs`
- **Linking**: Hybrid LLD approach in `src/main.rs`

## Troubleshooting

**Error: "No LLD linker found"**
- Ensure Rust is properly installed via rustup
- Try: `rustc --print target-libdir` (should work)

**Compilation works but linking fails**
- This is expected for development mode in some environments
- The CI/release packages will have proper bundled LLD

## Release Process

Releases are automated via GitHub Actions:
- Push a tag: `git tag v0.1.0 && git push origin v0.1.0`
- GitHub Actions builds cross-platform packages with bundled LLD
- End-users get standalone packages requiring no external dependencies

## Questions?

Open an issue or discussion on GitHub!