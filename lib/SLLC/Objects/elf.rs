pub struct ELFWriter;

impl ELFWriter {
    pub fn write(_machine_code: &[u8]) -> Result<Vec<u8>, String> {
        Err("ELF object emission is planned for the SLLC object writer".into())
    }
}
