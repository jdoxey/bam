# Compiler Back-end Options

Using an existing compiler back-end (LLVM or GCC) requires anyone wanting to compile a `bam` program to have these complex packages installed. Maybe the `bam` compiler could just include the necessary libraries from LLVM/GCC? Maybe they could run in Docker? Debugging might be tricky.

## Option Chosen: TBD

## Option 1: LLVM

### PROs
- Works on Linux, Windows and macOS
- Battle hardened from Apple's input into the project (especially for ARM)
- Very capable of producing binaries for many platforms (probably heavier ARM focus than others)
- Intermediate Language makes it awesome for separating front-end and back-end, and producing machine code at a later stage.

### CONs
- `bam` users need LLVM installed (or available). Maybe the `bam` compiler could just include the necessary libraries from LLVM?
- C++ can be esoteric to work with (reasonably high ramp-up for new developers)

## Option 2: GCC

### PROs
- Battle hardened
- May have better support for edge case architectures?

### CONs
- `bam` users need GCC installed (or available). Maybe the `bam` compiler could just include the necessary libraries from GCC?
- Not sure what the GIMBLE intermediate representation is like

## Option 3: Write generators for major assembly types

### PROs
- No extra packages required for people wanting to compile `bam` programs
### CONs
- A lot of work
