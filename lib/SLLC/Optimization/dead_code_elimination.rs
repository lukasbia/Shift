use crate::core::{Instruction, Module, Value};
use super::pass_manager::Pass;

pub struct DeadCodeElimination;

impl Pass for DeadCodeElimination {
    fn name(&self) -> &'static str { "dead-code-elimination" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            let mut result = Vec::with_capacity(function.instructions.len());

            for instruction in function.instructions.drain(..) {
                match instruction {
                    Instruction::Nop => {}
                    Instruction::Move { dst, src: Value::Register(src) } if dst == src => {}
                    other => result.push(other),
                }
            }

            function.instructions = result;
        }
    }
}
