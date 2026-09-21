use crate::core::{Instruction, Module};
use super::pass_manager::Pass;

pub struct SimplifyCFG;

impl Pass for SimplifyCFG {
    fn name(&self) -> &'static str { "simplify-cfg" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            function.instructions.dedup_by(|a, b| {
                matches!((a, b), (Instruction::Label(a), Instruction::Label(b)) if a == b)
            });
        }
    }
}
