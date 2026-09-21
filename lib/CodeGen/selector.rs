use crate::diagnostics::CodeGenDiagnostics;
use crate::machine::{MachineModule,MachineOpcode};

pub trait InstructionSelector{
    fn select(&self,module:&MachineModule,diagnostics:&mut CodeGenDiagnostics)->MachineModule;
}
pub struct GenericInstructionSelector;
impl InstructionSelector for GenericInstructionSelector{
    fn select(&self,module:&MachineModule,diagnostics:&mut CodeGenDiagnostics)->MachineModule{
        let mut out=module.clone();
        for f in &mut out.functions{for b in &mut f.blocks{for i in &mut b.instructions{
            if i.opcode==MachineOpcode::Custom("unsupported".into()){
                diagnostics.warning("unsupported custom instruction encountered");
            }
        }}}
        out
    }
}
