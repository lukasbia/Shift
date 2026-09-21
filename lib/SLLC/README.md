# SLLC

SLLC is the native low-level compiler/backend for Shift.

SLLC is written in Rust and does not use LLVM.

## Targets

- x86
- x86-64
- ARM32
- ARM64
- WASM32

## Output

- hexadecimal machine code
- raw machine code
- assembly
- WebAssembly

## Optimization

SLLC has its own optimization pipeline:

1. Constant folding
2. Constant propagation
3. Copy propagation
4. Instruction combining
5. CFG simplification
6. Dead-code elimination
7. Peephole optimization
8. Register optimization

Optimization levels:

```text
-O0
-O1
-O2
-O3
-Os
```

## Build

```text
cargo build
cargo test
cargo run -- --target x86_64 --emit hex -O2
cargo run -- --target arm64 --emit asm
cargo run -- --target wasm32 --emit wasm --output program.wasm
```

The object writers and native linkers are intentionally separated from the target encoders so they can be implemented independently.
