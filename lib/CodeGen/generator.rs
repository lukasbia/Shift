use crate::diagnostics::CodeGenDiagnostics;
use crate::layout::{DataLayout,Relocation};
use crate::lowering::{GenericTargetLowering,TargetLowering};
use crate::machine::MachineModule;
use crate::options::CodeGenOptions;
use crate::registers::RegisterAllocator;
use crate::selector::{GenericInstructionSelector,InstructionSelector};
use crate::target::TargetTriple;

#[derive(Clone,Debug)]
pub struct GeneratedCode{
    pub target:TargetTriple,pub module:MachineModule,
    pub relocations:Vec<Relocation>,pub data_layout:DataLayout,
}
pub struct CodeGenerator{
    pub options:CodeGenOptions,pub diagnostics:CodeGenDiagnostics,
    pub register_allocator:RegisterAllocator,
    selector:Box<dyn InstructionSelector>,lowering:Box<dyn TargetLowering>,
}
impl CodeGenerator{
    pub fn new(options:CodeGenOptions)->Self{
        Self{options,diagnostics:CodeGenDiagnostics::default(),
            register_allocator:RegisterAllocator::new(),
            selector:Box::new(GenericInstructionSelector),
            lowering:Box::new(GenericTargetLowering)}
    }
    pub fn generate(&mut self,module:MachineModule)->Result<GeneratedCode,String>{
        let mut lowered=self.selector.select(&module,&mut self.diagnostics);
        self.lowering.lower_module(&mut lowered,&mut self.diagnostics);
        if self.diagnostics.has_errors(){
            return Err(self.diagnostics.errors.iter().map(|e|e.message.clone()).collect::<Vec<_>>().join("\n"));
        }
        Ok(GeneratedCode{target:self.options.target.clone(),module:lowered,
            relocations:vec![],data_layout:DataLayout::for_target(&self.options.target)})
    }
}
pub fn create_host_code_generator()->CodeGenerator{CodeGenerator::new(CodeGenOptions::default())}
pub fn create_code_generator(target:TargetTriple)->CodeGenerator{
    CodeGenerator::new(CodeGenOptions{target,..CodeGenOptions::default()})
}
