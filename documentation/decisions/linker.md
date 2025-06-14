# Linker Choice for Bam

## Background

Cranelift generates object files (.o/.obj) but does not handle the linking phase to create executable binaries. We need a cross-platform linking solution that doesn't require bam programmers to install separate compiler toolchains on Linux, macOS, and Windows.

## Requirements

- **Cross-platform**: Must work on Linux, macOS, and Windows
- **No external dependencies**: Users shouldn't need to install GCC, Clang, or MSVC
- **Self-contained**: The bam compiler should handle the entire compilation pipeline
- **Performance**: Linking should be fast
- **Reliability**: Must be battle-tested and stable

## Options Considered

### 1. **LLD (LLVM Linker)**

LLD is the modern linker from the LLVM project, designed specifically for cross-platform use.

**Implementation approaches:**
- Ship LLD binaries with bam compiler distribution
- Call LLD as subprocess from bam compiler
- Potential future: Embed via `lld-rs` crate (if/when available)

**Pros:**
- Cross-platform (Linux, macOS, Windows)
- Fast, modern linker designed for this exact use case
- Used by Rust itself and many other language implementations
- Battle-tested and actively maintained
- Can handle all target platforms from any host platform
- Self-contained when shipped with compiler

**Cons:**
- Adds ~10-50MB to compiler distribution size
- Requires subprocess management
- Need to ship separate binaries for each host platform

### 2. **System Linkers (GCC/Clang/MSVC)**

Use whatever linker is available on the target system.

**Pros:**
- No additional distribution size
- Leverages existing, well-tested toolchains
- Zero implementation complexity

**Cons:**
- ❌ Requires external dependencies (violates core requirement)
- ❌ Different behavior across platforms
- ❌ Complex detection logic needed
- ❌ Poor user experience (installation friction)

### 3. **mold (Linux/macOS) + LLD (Windows)**

Use the extremely fast `mold` linker on Unix systems and LLD on Windows.

**Pros:**
- Excellent performance (mold is fastest available linker)
- Still cross-platform
- Can ship binaries with compiler

**Cons:**
- More complex distribution (multiple linkers)
- mold is newer/less battle-tested than LLD
- Additional maintenance burden

### 4. **Pure Rust Object File Merging**

Implement linking entirely in Rust using crates like `object` to directly merge object files.

**Pros:**
- No external binaries needed
- Rust-native solution
- Potentially smallest distribution size
- Complete control over linking process

**Cons:**
- ❌ Extremely complex to implement correctly
- ❌ Need to handle relocations, symbols, debugging info, etc.
- ❌ Essentially reimplementing a linker from scratch
- ❌ High risk of bugs and compatibility issues
- ❌ Significant development time investment

### 5. **WebAssembly + WASI**

Compile to WebAssembly instead of native code, eliminating the need for linking.

**Pros:**
- Truly universal binaries
- No linking phase needed
- Maximum portability

**Cons:**
- ❌ Performance overhead
- ❌ Limited system integration capabilities
- ❌ Doesn't align with bam's goals as a systems language
- ❌ Complex FFI for native library integration

## Comparison Matrix

| Solution | Cross-Platform | No External Deps | Performance | Complexity | Distribution Size | Maintenance |
|----------|---------------|------------------|-------------|------------|------------------|-------------|
| **LLD** | ✅ Excellent | ✅ Yes (shipped) | ✅ Fast | ✅ Low | ⚠️ Medium (+10-50MB) | ✅ Low |
| System Linkers | ✅ Yes | ❌ No | ✅ Fast | ⚠️ Medium | ✅ Zero | ⚠️ Medium |
| mold + LLD | ✅ Yes | ✅ Yes (shipped) | ✅ Fastest | ⚠️ Medium | ❌ Large (+100MB) | ❌ High |
| Pure Rust | ✅ Yes | ✅ Yes | ⚠️ Unknown | ❌ Very High | ✅ Small | ❌ Very High |
| WebAssembly | ✅ Excellent | ✅ Yes | ❌ Slower | ⚠️ Medium | ✅ Small | ⚠️ Medium |

## Decision: LLD

**Chosen approach: Ship LLD binaries with bam compiler distribution**

### Rationale

1. **Meets all core requirements**: Cross-platform, no external dependencies when shipped with compiler, reliable
2. **Proven solution**: Used by Rust, Swift, and other modern language implementations
3. **Best balance**: Good performance, low complexity, acceptable distribution size
4. **Future-proof**: Can potentially embed as library later if `lld-rs` becomes available

### Implementation Plan

**Phase 1 (Current)**: Use system GCC/Clang for development and testing
**Phase 2 (Distribution)**: 
- Include LLD binaries for Linux, macOS, and Windows in bam releases
- Automatic detection and execution of appropriate LLD binary
- Fallback to system linker if LLD not available (for development builds)

**Phase 3 (Future optimization)**:
- Investigate embedding LLD as library to eliminate subprocess overhead
- Consider link-time optimization integration

---

**Decision Date**: 2025-06-14  
**Status**: Approved  
**Next Review**: When implementing distribution packaging