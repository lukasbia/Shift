use super::linker::Linker;

pub struct MachOLinker;

impl Linker for MachOLinker {
    fn link(&self, _objects: &[Vec<u8>]) -> Result<Vec<u8>, String> {
        Err("Mach-O linking is planned for the SLLC linker".into())
    }
}
