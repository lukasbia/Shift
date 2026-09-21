use crate::core::Register;

#[derive(Default)]
pub struct RegisterAllocator;

impl RegisterAllocator {
    pub fn allocate(&self, virtual_register: Register) -> Register {
        virtual_register
    }
}
