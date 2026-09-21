use crate::core::{Instruction, Module, Register, Value};
use super::pass_manager::Pass;
use std::collections::HashMap;

pub struct ConstantPropagation;

impl Pass for ConstantPropagation {
    fn name(&self) -> &'static str { "constant-propagation" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            let mut constants: HashMap<Register, i64> = HashMap::new();

            for instruction in &mut function.instructions {
                match instruction {
                    Instruction::Move { dst, src: Value::Immediate(value) } => {
                        constants.insert(*dst, *value);
                    }
                    Instruction::Move { dst, src: Value::Register(source) } => {
                        if let Some(value) = constants.get(source).copied() {
                            *instruction = Instruction::Move {
                                dst: *dst,
                                src: Value::Immediate(value),
                            };
                            constants.insert(*dst, value);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
