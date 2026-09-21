use crate::target::TargetTriple;

#[derive(Clone,Debug)]
pub struct CodeGenOptions{
    pub target:TargetTriple,pub optimize:bool,pub emit_debug_information:bool,
    pub emit_comments:bool,pub position_independent_code:bool,pub stack_protector:bool,
}
impl Default for CodeGenOptions{
    fn default()->Self{Self{target:TargetTriple::host(),optimize:true,
        emit_debug_information:false,emit_comments:true,
        position_independent_code:false,stack_protector:false}}
}
