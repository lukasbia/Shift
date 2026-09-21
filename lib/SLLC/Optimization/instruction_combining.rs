use crate::core::{Instruction, Module, Value};
use super::pass_manager::Pass;

pub struct InstructionCombining;

impl Pass for InstructionCombining {
    fn name(&self) -> &'static str { "instruction-combining" }

    fn run(&self, module: &mut Module) {
        for function in &mut module.functions {
            let mut result = Vec::new();
            let mut index = 0;

            while index < function.instructions.len() {
                if index + 1 < function.instructions.len() {
                    if let (
                        Instruction::Move { dst: first_dst, src: first_src },
                        Instruction::Move { dst: second_dst, src: Value::Register(first_reg) },
                    ) = (&function.instructions[index], &function.instructions[index + 1]) {
                        if *first_reg == *first_dst {
                            result.push(Instruction::Move {
                                dst: *second_dst,
                                src: *first_src,
                            });
                            index += 2;
                            continue;
                        }
                    }
                }

                result.push(function.instructions[index].clone());
                index += 1;
            }

            function.instructions = result;
        }
    }
}
