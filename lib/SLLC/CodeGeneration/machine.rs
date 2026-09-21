use crate::core::{Register, Value};

#[derive(Clone, Debug)]
pub struct MachineFunction {
    pub name: String,
    pub instructions: Vec<MachineInstruction>,
}

#[derive(Clone, Debug)]
pub enum MachineInstruction {
    Nop,
    Move(Register, Value),
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Ret,
}
