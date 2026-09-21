pub mod target;
pub mod registers;
pub mod machine;
pub mod layout;
pub mod options;
pub mod diagnostics;
pub mod selector;
pub mod lowering;
pub mod generator;

pub use target::*;
pub use registers::*;
pub use machine::*;
pub use layout::*;
pub use options::*;
pub use diagnostics::*;
pub use selector::*;
pub use lowering::*;
pub use generator::*;

pub fn create_module(name:impl Into<String>)->MachineModule{MachineModule::new(name)}
pub fn create_function(module:&mut MachineModule,name:impl Into<String>)->usize{
    module.add_function(MachineFunction::new(name));module.functions.len()-1
}
pub fn emit_target_description(target:&TargetTriple)->String{
    format!("architecture={}\nos={}\ntriple={}\npointer_width={}",
        target.architecture,target.operating_system,target.triple_string(),
        target.architecture.pointer_width())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn target_exists(){assert!(!TargetTriple::host().triple_string().is_empty());}
    #[test]fn registers_unique(){let mut a=RegisterAllocator::new();assert_ne!(a.create_virtual(),a.create_virtual());}
    #[test]fn module_functions(){let mut m=create_module("test");create_function(&mut m,"main");assert_eq!(m.functions.len(),1);}
    #[test]fn empty_block_gets_nop(){
        let mut m=create_module("test");create_function(&mut m,"main");
        let mut g=create_host_code_generator();let r=g.generate(m).unwrap();
        assert!(!r.module.functions[0].blocks[0].instructions.is_empty());
    }
}
