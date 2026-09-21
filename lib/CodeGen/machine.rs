use crate::registers::{Register,VirtualRegister};

#[derive(Clone,Debug,PartialEq)]
pub enum MachineOperand{
    Register(Register), Immediate(i64), UnsignedImmediate(u64), Float(f64),
    Symbol(String), Label(String),
    Memory{base:Option<Register>,index:Option<Register>,scale:u8,displacement:i64},
}
impl MachineOperand{
    pub fn register(v:Register)->Self{Self::Register(v)}
    pub fn immediate(v:i64)->Self{Self::Immediate(v)}
    pub fn symbol(v:impl Into<String>)->Self{Self::Symbol(v.into())}
}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum MachineOpcode{
    Nop,Move,Load,Store,Add,Sub,Mul,Div,Mod,And,Or,Xor,ShiftLeft,ShiftRight,
    Compare,Branch,BranchEqual,BranchNotEqual,BranchLess,BranchGreater,
    Call,Return,Push,Pop,Retain,Release,Trap,Syscall,Convert,Select,Phi,
    Custom(String),
}
#[derive(Clone,Debug)]
pub struct MachineInstruction{
    pub opcode:MachineOpcode,
    pub operands:Vec<MachineOperand>,
    pub comment:Option<String>,
}
impl MachineInstruction{
    pub fn new(opcode:MachineOpcode,operands:Vec<MachineOperand>)->Self{
        Self{opcode,operands,comment:None}
    }
    pub fn comment(mut self,v:impl Into<String>)->Self{self.comment=Some(v.into());self}
}
#[derive(Clone,Debug)]
pub struct MachineBasicBlock{
    pub name:String,pub instructions:Vec<MachineInstruction>,
    pub predecessors:Vec<String>,pub successors:Vec<String>,
}
impl MachineBasicBlock{
    pub fn new(v:impl Into<String>)->Self{
        Self{name:v.into(),instructions:vec![],predecessors:vec![],successors:vec![]}
    }
    pub fn append(&mut self,i:MachineInstruction){self.instructions.push(i)}
    pub fn add_successor(&mut self,v:impl Into<String>){let v=v.into();if !self.successors.contains(&v){self.successors.push(v)}}
    pub fn add_predecessor(&mut self,v:impl Into<String>){let v=v.into();if !self.predecessors.contains(&v){self.predecessors.push(v)}}
}
#[derive(Clone,Debug)]
pub struct MachineFunction{
    pub name:String,pub blocks:Vec<MachineBasicBlock>,
    pub virtual_registers:Vec<VirtualRegister>,pub stack_size:u64,
    pub calling_convention:CallingConvention,
}
impl MachineFunction{
    pub fn new(v:impl Into<String>)->Self{
        Self{name:v.into(),blocks:vec![MachineBasicBlock::new("entry")],
        virtual_registers:vec![],stack_size:0,calling_convention:CallingConvention::Default}
    }
    pub fn new_virtual_register(&mut self)->VirtualRegister{
        let r=VirtualRegister::new(self.virtual_registers.len() as u32);
        self.virtual_registers.push(r.clone());r
    }
    pub fn add_block(&mut self,v:impl Into<String>)->usize{self.blocks.push(MachineBasicBlock::new(v));self.blocks.len()-1}
}
#[derive(Clone,Debug,Default)]
pub struct MachineModule{pub name:String,pub functions:Vec<MachineFunction>,pub globals:Vec<String>}
impl MachineModule{
    pub fn new(v:impl Into<String>)->Self{Self{name:v.into(),..Default::default()}}
    pub fn add_function(&mut self,f:MachineFunction){self.functions.push(f)}
}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum CallingConvention{Default,C,Swift,SystemV,Windows,AAPCS,Wasm}
