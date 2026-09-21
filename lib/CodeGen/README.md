# Shift Code Generation

Rust code-generation layer for Shift.

Pipeline:

Shift -> Swift SIL -> Swift SIL Optimization -> Rust Code Generation -> SLLC

The code-generation layer is split into separate Rust modules for target
handling, registers, machine IR, data layout, options, diagnostics,
instruction selection, lowering, and the main generator.
