use super::Instruction;

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub instructions: Vec<Instruction>,
}
