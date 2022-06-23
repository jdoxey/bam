# `bam` Roadmap

This project will need to be built in stages. Existing tools and libraries (LLVM, GCC, etc) will need to be leveraged before having a complete native `bam` compiler toolchain.

*This is a pretty rough brain dump. To be adjusted as required.*

# Stage 1: Flex/Bison/C++/LLVM

`source.bam -> flex/bison -> C++ AST -> LLVM APIs -> executable (for host architecture)`

Source is parsed by flex (in C mode) and bison in C++ mode. Bison generates a c++ AST. C++ code uses LLVM libraries to generate platform specific object files and/or linked executables. No cross compilation, i.e. compiler only produces binaries for the host architecture, so they can only be run on systems with similar architecture.

# Stage 2: Language Backward Compatability

Source code written for older versions of `bam` will also compile with new compilers.

# Stage 3: ABI Stability

Libraries compiled with an older version of `bam` will still link with newer versions.

# Stage 4: `bam` Scanner/Parser and AST, LLVM Code Generation

`source.bam -> bam scanner/parser -> bam AST -> LLVM -> executable`

Not sure how to get from bam AST in memory to LLVM for assembling, maybe go via IR.

# Stage 5: `bam` All The Way Down

`source.bam -> bam scanner/parser -> bam AST -> magic -> executable`

Holy Grail. May not ever be totally necessary. Not sure magic is real.
