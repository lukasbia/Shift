use super::{Register, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Nop,
    Move { dst: Register, src: Value },
    Add { dst: Register, lhs: Value, rhs: Value },
    Sub { dst: Register, lhs: Value, rhs: Value },
    Mul { dst: Register, lhs: Value, rhs: Value },
    Div { dst: Register, lhs: Value, rhs: Value },
    And { dst: Register, lhs: Value, rhs: Value },
    Or { dst: Register, lhs: Value, rhs: Value },
    Xor { dst: Register, lhs: Value, rhs: Value },
    Compare { lhs: Value, rhs: Value },
    Jump(String),
    Call(String),
    Return(Value),
    Label(String),
}
