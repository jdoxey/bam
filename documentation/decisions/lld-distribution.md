# LLD Distribution Strategy

## Problem

The bam compiler needs to provide linking functionality to create executable binaries from object files. For version 0.1, the roadmap specifies that `bam hello.bam` should "compile and link it into the final executable" that end-users can run immediately. This requires a linking solution that works on systems without development tools installed.

## Options Considered

### Option 1: Embed LLD Library (Static Linking)

**Approach**: Statically link LLD as a library and call `lld::lldMain()` directly from Rust code.

**Attempted Implementation**:
- Tried `lld_rs = "140.0.0"` crate
- Tried `lld-rx = "0.1.1"` crate  
- Tried `mun_lld = "110.0.0"` crate
- Tried `lld-sys = "0.1.0"` crate

**Issues Encountered**:
- **LLVM Version Compatibility**: All crates had compilation issues with LLVM 14
- **C++ Template Errors**: `std::unique_lock lock(concurrencyMutex)` missing template arguments
- **Missing Libraries**: `error: could not find native static library 'Polly'`
- **API Compatibility**: Function signature mismatches between crate versions and installed LLVM

**Advantages**:
- ✅ True standalone binary
- ✅ No external dependencies
- ✅ Fastest linking (in-process)
- ✅ Complete control over linking behavior

**Disadvantages**:
- ❌ Complex C++ bindings with version compatibility issues
- ❌ Requires specific LLVM version alignment
- ❌ Build-time dependency on LLVM development libraries

**Status**: **Attempted but abandoned** due to C++ compatibility issues.

### Option 2: Use rust-lld (Shipped with Rust)

**Approach**: Call the `rust-lld` binary that ships with every Rust installation.

**Implementation**:
```rust
// Find rust-lld from current toolchain
if let Ok(output) = process::Command::new("rustc").arg("--print").arg("target-libdir").output() {
    let target_libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let rust_lld = Path::new(&target_libdir).join("../bin/rust-lld");
    // Call rust-lld with proper arguments
}
```

**Advantages**:
- ✅ No additional dependencies during build
- ✅ Cross-platform (ships with Rust on all platforms)
- ✅ Well-tested and maintained by Rust team
- ✅ Fast linking performance

**Disadvantages**:
- ❌ **Requires Rust installation on end-user systems**
- ❌ Violates v0.1 requirement: "base install of their OS, i.e. no compiler components installed"
- ❌ External process overhead

**Status**: **Implemented and working**, but **rejected** for v0.1 due to Rust dependency requirement.

### Option 3: Distribute LLD Binary with bam

**Approach**: Include the LLD binary in the bam distribution package, alongside the bam executable.

**Distribution Structure**:
```
bam-0.1.0.zip
├── bin/
│   ├── bam                 # Main compiler executable
│   └── ld.lld              # Bundled LLD linker
├── LICENSE                 # Combined license notices
└── README.md               # Installation instructions
```

**Implementation Strategy**:
```rust
fn get_bundled_lld() -> Result<PathBuf, String> {
    // Look for ld.lld in same directory as bam executable
    let exe_path = env::current_exe()
        .map_err(|e| format!("Cannot determine executable path: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Cannot find executable directory")?;
    let lld_path = exe_dir.join("ld.lld");
    
    if lld_path.exists() {
        Ok(lld_path)
    } else {
        Err(format!(
            "Bundled LLD linker not found at: {}\n\
            This indicates a corrupted or incomplete bam installation.\n\
            Please re-download the complete bam distribution package.",
            lld_path.display()
        ))
    }
}
```

**Advantages**:
- ✅ **No external dependencies for end-users**
- ✅ **Normal-sized bam executable** (no embedded binary bloat)
- ✅ Cross-platform (include platform-specific LLD in each package)
- ✅ Self-contained distribution package
- ✅ Works on "base install" systems
- ✅ **Clean separation of concerns**
- ✅ Easy to update LLD independently
- ✅ **Deterministic behavior** - no confusing fallback chains
- ✅ **Easy issue triaging** - single linker path to debug

**Disadvantages**:
- ❌ Multiple files to distribute (but in single package)
- ❌ Need platform-specific distributions
- ❌ **No fallback options** - fails if LLD missing or broken

**Status**: **Recommended solution** for v0.1.

### Option 4: System Linker Fallback

**Approach**: Shell out to system linkers (gcc, ld, etc.) when available.

**Implementation**:
```rust
fn link_with_system_linker(object_file: &str, executable_name: &str) -> Result<(), String> {
    let output = process::Command::new("gcc")
        .arg("-o").arg(executable_name)
        .arg(object_file)
        .output()?;
    // Handle result
}
```

**Advantages**:
- ✅ Works on systems with development tools
- ✅ No additional binary size
- ✅ Uses optimized system linkers

**Disadvantages**:
- ❌ **Requires development tools on end-user systems**
- ❌ Violates v0.1 "base install" requirement
- ❌ Inconsistent behavior across platforms
- ❌ Different linker capabilities and flags per platform

**Status**: **Implemented as fallback** but not suitable for v0.1 primary solution.

## License Considerations

**LLD License**: Apache License 2.0 with LLVM Exceptions

### Key License Benefits:
- ✅ **No attribution required for binary distribution**: LLVM exception allows embedding without attribution
- ✅ **Commercial use allowed**: Can be used in commercial products
- ✅ **Permissive**: Compatible with any license for bam itself
- ✅ **No copyleft**: No requirement to open-source bam

### Requirements:
- **Minimal**: Include LLD copyright notice in distribution (LICENSE file)
- **Don't strip**: LLVM copyright headers if modifying LLD source

### License Notice:
```
This software includes LLD from the LLVM Project.
Copyright (c) 2003-2019 University of Illinois at Urbana-Champaign.
Licensed under the Apache License 2.0 with LLVM Exceptions.
See: https://llvm.org/LICENSE.txt
```

## Decision

**Selected**: **Option 3 - Bundle LLD Binary**

### Rationale:
1. **V0.1 Compliance**: Meets requirement for "base install of OS, no compiler components"
2. **Standalone**: Single `bam` executable contains everything needed
3. **License Compatible**: Apache 2.0 with LLVM Exceptions allows bundling
4. **Cross-platform**: Can bundle platform-specific LLD binaries
5. **Reliable**: Avoids C++ binding complexity and version compatibility issues

### Implementation Plan:
1. Extract `ld.lld` from current Rust installation during packaging
2. Create distribution packages with both `bam` and `ld.lld` binaries
3. Update bam to look for `ld.lld` in same directory as executable
4. **Fail with clear error message** if bundled LLD not found (no fallbacks)
5. Include appropriate license attribution in package

### Trade-offs Accepted:
- **Multiple Files**: Distribution contains multiple binaries (but packaged together)
- **Platform-specific Packages**: Need separate downloads for Linux/macOS/Windows
- **Packaging Complexity**: Need build process to create proper distributions
- **No Fallbacks**: Stricter failure mode improves issue triaging but less forgiving

This decision enables bam to be a truly standalone compiler with **deterministic, predictable behavior**. Users get clear error messages if something is wrong, making issues easier to diagnose and fix. This meets the core v0.1 roadmap requirement while maintaining reliability.