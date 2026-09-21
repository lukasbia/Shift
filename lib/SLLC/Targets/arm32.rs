use crate::core::{Instruction, Module, Value};

pub fn emit(kind: &str, module: &Module) -> Result<Vec<u8>, String> {
    let bytes = lower(module)?;
    match kind {
        "hex" => Ok(to_hex(&bytes).into_bytes()),
        "raw" => Ok(bytes),
        "asm" => Ok("; ARM32 assembly\n".as_bytes().to_vec()),
        _ => Err(format!("unsupported ARM32 output: {kind}")),
    }
}

fn lower(module: &Module) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();

    for function in &module.functions {
        for instruction in &function.instructions {
            match instruction {
                Instruction::Nop => emit_word(&mut out, 0xE1A0_0000),
                Instruction::Move { dst, src: Value::Immediate(value) } if *value >= 0 && *value <= 255 => {
                    if dst.0 > 15 { return Err("ARM32 register out of range".into()); }
                    emit_word(&mut out, 0xE3A0_0000 | (dst.0 << 12) | *value as u32);
                }
                Instruction::Return(_) => emit_word(&mut out, 0xE12F_FF1E),
                _ => return Err("ARM32 instruction lowering is not implemented for this instruction".into()),
            }
        }
    }

    Ok(out)
}

fn emit_word(out: &mut Vec<u8>, word: u32) {
    out.extend_from_slice(&word.to_le_bytes());
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}
