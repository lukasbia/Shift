pub mod core;
pub mod frontend;
pub mod optimization;
pub mod codegen;
pub mod targets;
pub mod assembly;
pub mod object;
pub mod linker;

pub use core::{Module, Function, Instruction, Value, Register};
pub use targets::Target;
pub use optimization::PassManager;
