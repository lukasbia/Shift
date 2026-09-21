use crate::diagnostics::CodeGenDiagnostics;
use crate::machine::{MachineModule,MachineOpcode,MachineInstruction};

pub trait TargetLowering{
    fn lower_module(&self,module:&mut MachineModule,diagnostics:&mut CodeGenDiagnostics);
}
pub struct GenericTargetLowering;
impl TargetLowering for GenericTargetLowering{
    fn lower_module(&self,module:&mut MachineModule,diagnostics:&mut CodeGenDiagnostics){
        for f in &mut module.functions{
            if f.blocks.is_empty(){diagnostics.error(format!("function '{}' has no basic blocks",f.name));}
            for b in &mut f.blocks{
                if b.instructions.is_empty(){b.append(MachineInstruction::new(MachineOpcode::Nop,vec![]));}
            }
        }
    }
}
