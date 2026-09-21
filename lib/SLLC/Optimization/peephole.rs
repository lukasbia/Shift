use crate::core::{Instruction, Module, Register, Value};
use super::pass_manager::Pass;

pub struct Peephole;

impl Pass for Peephole {
    fn name(&self) -> &'static str { "peephole" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            for instruction in &mut function.instructions {
                if let Instruction::Add { dst, lhs: Value::Register(lhs), rhs: Value::Immediate(0) } = instruction {
                    *instruction = Instruction::Move {
                        dst: *dst,
                        src: Value::Register(*lhs),
                    };
                }
            }
        }
    }
}

#[allow(dead_code)]
fn _keep_register_type(_: Register) {}
