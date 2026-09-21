use crate::core::Module;

pub trait Pass {
    fn name(&self) -> &'static str;
    fn run(&self, module: &mut Module);
}

pub struct PassManager {
    passes: Vec<Box<dyn Pass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add<P: Pass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&self, module: &mut Module) {
        for pass in &self.passes {
            pass.run(module);
        }
    }

    pub fn for_level(level: &str) -> Self {
        let mut manager = Self::new();

        match level {
            "0" => {}
            "1" => {
                manager.add(crate::optimization::constant_folding::ConstantFolding);
                manager.add(crate::optimization::copy_propagation::CopyPropagation);
            }
            "s" => {
                manager.add(crate::optimization::constant_folding::ConstantFolding);
                manager.add(crate::optimization::dead_code_elimination::DeadCodeElimination);
                manager.add(crate::optimization::peephole::Peephole);
            }
            _ => {
                manager.add(crate::optimization::constant_folding::ConstantFolding);
                manager.add(crate::optimization::constant_propagation::ConstantPropagation);
                manager.add(crate::optimization::copy_propagation::CopyPropagation);
                manager.add(crate::optimization::instruction_combining::InstructionCombining);
                manager.add(crate::optimization::simplify_cfg::SimplifyCFG);
                manager.add(crate::optimization::dead_code_elimination::DeadCodeElimination);
                manager.add(crate::optimization::peephole::Peephole);
                manager.add(crate::optimization::register_optimization::RegisterOptimization);
            }
        }

        manager
    }
}
