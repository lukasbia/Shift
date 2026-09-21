pub struct PEWriter;

impl PEWriter {
    pub fn write(_machine_code: &[u8]) -> Result<Vec<u8>, String> {
        Err("PE/COFF object emission is planned for the SLLC object writer".into())
    }
}
