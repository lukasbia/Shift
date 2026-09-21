use crate::core::Module;
use std::fmt;

#[derive(Debug)]
pub struct SILReadError(pub String);

impl fmt::Display for SILReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SILReadError {}

pub fn read_sil(_source: &str) -> Result<Module, SILReadError> {
    Err(SILReadError("SIL reader grammar is not implemented yet".into()))
}
