#[derive(Clone,Debug)]
pub struct CodeGenDiagnostic{
    pub message:String,pub function:Option<String>,pub block:Option<String>,
}
impl CodeGenDiagnostic{
    pub fn new(v:impl Into<String>)->Self{Self{message:v.into(),function:None,block:None}}
}
#[derive(Clone,Debug,Default)]
pub struct CodeGenDiagnostics{pub errors:Vec<CodeGenDiagnostic>,pub warnings:Vec<CodeGenDiagnostic>}
impl CodeGenDiagnostics{
    pub fn error(&mut self,v:impl Into<String>){self.errors.push(CodeGenDiagnostic::new(v))}
    pub fn warning(&mut self,v:impl Into<String>){self.warnings.push(CodeGenDiagnostic::new(v))}
    pub fn has_errors(&self)->bool{!self.errors.is_empty()}
}
