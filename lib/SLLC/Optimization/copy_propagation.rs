use crate::core::{Instruction, Module, Value};
use super::pass_manager::Pass;

pub struct CopyPropagation;

impl Pass for CopyPropagation {
    fn name(&self) -> &'static str { "copy-propagation" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            let mut copies = Vec::new();

            for instruction in &function.instructions {
                if let Instruction::Move { dst, src: Value::Register(src) } = instruction {
                    copies.push((*dst, *src));
                }
            }

            for instruction in &mut function.instructions {
                replace_values(instruction, &copies);
            }
        }
    }
}

fn replace_values(instruction: &mut Instruction, copies: &[(crate::core::Register, crate::core::Register)]) {
    fn replace(value: &mut Value, copies: &[(crate::core::Register, crate::core::Register)]) {
        if let Value::Register(reg) = value {
            if let Some((_, source)) = copies.iter().find(|(destination, _)| destination == reg) {
                *value = Value::Register(*source);
            }
        }
    }

    match instruction {
        Instruction::Add { lhs, rhs, .. } |
        Instruction::Sub { lhs, rhs, .. } |
        Instruction::Mul { lhs, rhs, .. } |
        Instruction::Div { lhs, rhs, .. } |
        Instruction::And { lhs, rhs, .. } |
        Instruction::Or { lhs, rhs, .. } |
        Instruction::Xor { lhs, rhs, .. } => {
            replace(lhs, copies);
            replace(rhs, copies);
        }
        Instruction::Compare { lhs, rhs } => {
            replace(lhs, copies);
            replace(rhs, copies);
        }
        Instruction::Return(value) => replace(value, copies),
        Instruction::Move { src, .. } => replace(src, copies),
        _ => {}
    }
}
