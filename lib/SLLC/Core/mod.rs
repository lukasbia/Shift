mod module;
mod instruction;
mod value;
mod register;
mod types;

pub use module::{Module, Function};
pub use instruction::Instruction;
pub use value::Value;
pub use register::Register;
pub use types::SLLType;
