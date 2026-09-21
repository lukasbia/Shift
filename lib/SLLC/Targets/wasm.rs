use crate::core::{Instruction, Module, Value};

pub fn emit(kind: &str, module: &Module) -> Result<Vec<u8>, String> {
    let bytes = lower(module)?;

    match kind {
        "wasm" | "raw" => Ok(bytes),
        "hex" => Ok(to_hex(&bytes).into_bytes()),
        "asm" => Ok("; WebAssembly text emission\n".as_bytes().to_vec()),
        _ => Err(format!("unsupported WASM output: {kind}")),
    }
}

fn lower(module: &Module) -> Result<Vec<u8>, String> {
    // Minimal valid WASM module with a main function.
    // Future SLLC stages can lower the complete SLL instruction set.
    let mut code = Vec::new();

    for function in &module.functions {
        for instruction in &function.instructions {
            match instruction {
                Instruction::Nop => code.push(0x01),
                Instruction::Move { src: Value::Immediate(value), .. } if *value >= -128 && *value <= 127 => {
                    code.push(0x41);
                    code.push(*value as i8 as u8);
                }
                Instruction::Add { .. } => code.push(0x6A),
                Instruction::Return(_) => code.push(0x0F),
                _ => {}
            }
        }
    }

    code.push(0x0B);

    let mut module_bytes = Vec::new();
    module_bytes.extend_from_slice(b"\\0asm");
    module_bytes.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
    module_bytes.extend_from_slice(&[0x01, 0x04, 0x01, 0x60, 0x00, 0x00]);
    module_bytes.extend_from_slice(&[0x03, 0x02, 0x01, 0x00]);
    module_bytes.extend_from_slice(&[0x0A, code.len() as u8 + 2, 0x01, code.len() as u8]);
    module_bytes.extend_from_slice(&code);

    Ok(module_bytes)
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}
