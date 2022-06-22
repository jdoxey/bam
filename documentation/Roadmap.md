# `bam` Roadmap

This project will need to be built in stages. Existing tools and libraries (LLVM, GCC, etc) will need to be leveraged before having a complete native `bam` compiler toolchain.

1. source.bam -> flex/bison -> C++ AST -> LLVM APIs -> executable (for host architecture)

Source is parsed by flex (in c mode) and bison in c++ mode. Bison generates a c++ AST. C++ code uses LLVM libraries to generate platform specific object files and/or linked executables. No cross compilation, i.e. compiler only produces binaries for the host architecture, so they can only be run on systems with similar architecture.

1. Language backward compatability

Source code written for older versions of `bam` will also compile with new compilers.

1. ABI Stability

Libraries compiled with an older version of `bam` will still link with newer versions.

1. source.bam -> bam scanner/parser -> bam AST -> LLVM -> executable

Not sure how to get from bam AST in memory to LLVM for assembling, maybe IR.

1. source.bam -> bam scanner/parser -> bam AST -> magic -> executable

Holy Grail. May not ever be totally necessary. Not sure magic is real.
