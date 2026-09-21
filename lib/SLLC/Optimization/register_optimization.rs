use crate::core::Module;
use super::pass_manager::Pass;

pub struct RegisterOptimization;

impl Pass for RegisterOptimization {
    fn name(&self) -> &'static str { "register-optimization" }

    fn run(&self, _module: &mut Module) {
        // Target-specific register allocation happens after SLL optimization.
    }
}
