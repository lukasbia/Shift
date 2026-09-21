use super::Register;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    Immediate(i64),
    Register(Register),
    None,
}
