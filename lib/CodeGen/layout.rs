use crate::target::TargetTriple;

#[derive(Clone,Debug)]
pub struct DataLayout{
    pub pointer_size:u8,pub pointer_alignment:u8,pub integer_alignment:u8,pub stack_alignment:u32,
}
impl DataLayout{
    pub fn for_target(t:&TargetTriple)->Self{
        let p=t.architecture.pointer_width()/8;
        Self{pointer_size:p,pointer_alignment:p,integer_alignment:8,
            stack_alignment:match t.architecture{
                crate::target::TargetArchitecture::X86=>4,
                crate::target::TargetArchitecture::ARM32=>8,_=>16
            }}
    }
}
#[derive(Clone,Debug)]
pub struct Relocation{pub symbol:String,pub offset:u64,pub kind:String}
