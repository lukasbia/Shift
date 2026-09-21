pub struct MachOWriter;

impl MachOWriter {
    pub fn write(_machine_code: &[u8]) -> Result<Vec<u8>, String> {
        Err("Mach-O object emission is planned for the SLLC object writer".into())
    }
}
