# Compiler Back-end Options

## Option Chosen: Option 1 - LLVM

## Option 1: LLVM

### PROs
- Works on Linux, Windows and macOS
- Mature project (especially for ARM from iPhone usage)
- Very capable of producing binaries for many platforms 
- Intermediate representation makes it good for separating front-end and back-end, and producing machine code at a later stage.

### CONs
- People may need LLVM installed just to compile `bam` programs. Maybe the `bam` compiler could just include the necessary libraries from LLVM? - *TODO: verify this*
- C++ can be esoteric to work with (reasonably high ramp-up for new developers).

## Option 2: GCC

### PROs
- Mature project.
- May have better support for edge case architectures? - *TODO: verify this*

### CONs
- People may need GCC installed to compile `bam` programs. Maybe the `bam` compiler could just include the necessary libraries from GCC? - *TODO: verify this*
- Not sure what the GIMBLE intermediate representation is like

## Option 3: Write generators for major assembly types

### PROs
- No extra packages required for people wanting to compile `bam` programs
### CONs
- A lot of work
