pub trait Linker {
    fn link(&self, objects: &[Vec<u8>]) -> Result<Vec<u8>, String>;
}
