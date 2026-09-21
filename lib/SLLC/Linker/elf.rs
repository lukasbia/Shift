use super::linker::Linker;

pub struct ELFLinker;

impl Linker for ELFLinker {
    fn link(&self, _objects: &[Vec<u8>]) -> Result<Vec<u8>, String> {
        Err("ELF linking is planned for the SLLC linker".into())
    }
}
