pub mod x86;
pub mod x86_64;
pub mod arm32;
pub mod arm64;
pub mod wasm;

use crate::core::Module;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    X86,
    X86_64,
    ARM32,
    ARM64,
    WASM32,
}

impl Target {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "x86" | "i386" | "i686" => Some(Self::X86),
            "x86_64" | "x86-64" | "amd64" => Some(Self::X86_64),
            "arm32" | "arm" | "armv7" => Some(Self::ARM32),
            "arm64" | "aarch64" => Some(Self::ARM64),
            "wasm" | "wasm32" => Some(Self::WASM32),
            _ => None,
        }
    }
}

pub fn emit(target: &Target, kind: &str, module: &Module) -> Result<Vec<u8>, String> {
    match target {
        Target::X86 => x86::emit(kind, module),
        Target::X86_64 => x86_64::emit(kind, module),
        Target::ARM32 => arm32::emit(kind, module),
        Target::ARM64 => arm64::emit(kind, module),
        Target::WASM32 => wasm::emit(kind, module),
    }
}
