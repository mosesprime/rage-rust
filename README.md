# Rage-Rust

Bootstrap for the Rage programming language.

## How

Compiler (ragec) takes Rage General Language (RGL)(.rg) buffers/files or Rage Instruction Language (RIL)(.ri) buffers/files.
The compiler then tokenizes these input into temporary lexical tokens.
Temporary lexical tokens are then analyzed into semantic tokens.
Semantic tokens are structured into an abstract syntax tree (AST).
AST is analyzed to find errors.
AST used to parse an abstract instruction tree (AIT).
AIT is used for generalized optimization passes and/or sanitation.
Optimized AIT is then sent to output.
Output options include being passed to the assembler (ragea), serialized and piped, or writen to a file as assembly (.asm) or RIL (.ri).
Assembler takes an AIT as input.
Assembler can optimize AIT for specific platform features and capabilities.
Assembler then converts optimized AIT to platform specific machine code.
Assembler then links and forms final binary.
Assembler can also read machine code and reconstruct AIT.
