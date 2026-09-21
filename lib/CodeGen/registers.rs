use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct VirtualRegister{pub id:u32}
impl VirtualRegister{pub fn new(id:u32)->Self{Self{id}}}
impl fmt::Display for VirtualRegister{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"%v{}",self.id)}
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub struct PhysicalRegister{pub name:String}
impl PhysicalRegister{pub fn new(v:impl Into<String>)->Self{Self{name:v.into()}}}
impl fmt::Display for PhysicalRegister{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{}",self.name)}
}
#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Register{Virtual(VirtualRegister),Physical(PhysicalRegister)}
impl fmt::Display for Register{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self{Self::Virtual(v)=>write!(f,"{v}"),Self::Physical(v)=>write!(f,"{v}")}
    }
}
pub struct RegisterAllocator{
    next_virtual:u32,
    assignments:HashMap<VirtualRegister,PhysicalRegister>,
    used:HashSet<String>,
}
impl RegisterAllocator{
    pub fn new()->Self{Self{next_virtual:0,assignments:HashMap::new(),used:HashSet::new()}}
    pub fn create_virtual(&mut self)->VirtualRegister{
        let r=VirtualRegister::new(self.next_virtual);self.next_virtual+=1;r
    }
    pub fn assign(&mut self,v:VirtualRegister,p:PhysicalRegister){
        self.used.insert(p.name.clone());self.assignments.insert(v,p);
    }
    pub fn lookup(&self,v:&VirtualRegister)->Option<&PhysicalRegister>{self.assignments.get(v)}
}
