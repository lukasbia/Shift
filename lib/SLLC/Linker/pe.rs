use super::linker::Linker;

pub struct PELinker;

impl Linker for PELinker {
    fn link(&self, _objects: &[Vec<u8>]) -> Result<Vec<u8>, String> {
        Err("PE linking is planned for the SLLC linker".into())
    }
}
