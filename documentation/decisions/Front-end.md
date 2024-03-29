# Compiler Front-end Options

## Chosen Option: Flex/Bison

## Option 1: Flex/Bison
Might be a quick way to get started, with a view to eventually be replaced with hand-crafted parser (written in `bam`).

Requires the `bam` developer environment to have flex/bison installed. This should only be annoying for Windows, but Windows versions of flex/bison can be included in the `bam` repo.

### PROs
- Quick way to get started with reasonably complex language

### CONs
- Not a lot of tutorials and examples of using flex and bison with C++.
- Harder to customise error messages and suggestions for fixes than a hand-crafted parser.
- Need workarounds if coming up against any limitations of LALR(1).


## Option 2: Hand-crafted parser

### PROs
- Will be more flexible in the long run.
- Easier to provide better parser error messages and suggestions for fixes for errors.
- Easier to work around parser generator quirks/limitations.

### CONs
- More work than a parser generator.

## Option 3: Antlr

### PROs
- Seems feature rich and mature.

### CONs
- Adds Java dependency to the `bam` development environment.
