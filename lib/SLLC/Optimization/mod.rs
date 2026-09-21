pub mod pass_manager;
pub mod constant_folding;
pub mod constant_propagation;
pub mod dead_code_elimination;
pub mod copy_propagation;
pub mod instruction_combining;
pub mod simplify_cfg;
pub mod peephole;
pub mod register_optimization;

pub use pass_manager::PassManager;
