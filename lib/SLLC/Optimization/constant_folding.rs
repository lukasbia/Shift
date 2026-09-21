use crate::core::{Instruction, Module, Value};
use super::pass_manager::Pass;

pub struct ConstantFolding;

impl Pass for ConstantFolding {
    fn name(&self) -> &'static str { "constant-folding" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            for instruction in &mut function.instructions {
                let replacement = match instruction {
                    Instruction::Add { dst, lhs: Value::Immediate(a), rhs: Value::Immediate(b) } =>
                        Some(Instruction::Move { dst: *dst, src: Value::Immediate(*a + *b) }),
                    Instruction::Sub { dst, lhs: Value::Immediate(a), rhs: Value::Immediate(b) } =>
                        Some(Instruction::Move { dst: *dst, src: Value::Immediate(*a - *b) }),
                    Instruction::Mul { dst, lhs: Value::Immediate(a), rhs: Value::Immediate(b) } =>
                        Some(Instruction::Move { dst: *dst, src: Value::Immediate(*a * *b) }),
                    Instruction::Div { dst, lhs: Value::Immediate(a), rhs: Value::Immediate(b) } if *b != 0 =>
                        Some(Instruction::Move { dst: *dst, src: Value::Immediate(*a / *b) }),
                    _ => None,
                };

                if let Some(new_instruction) = replacement {
                    *instruction = new_instruction;
                }
            }
        }
    }
}
