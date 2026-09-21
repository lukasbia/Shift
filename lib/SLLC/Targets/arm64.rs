use crate::core::{Instruction, Module, Value};

pub fn emit(kind: &str, module: &Module) -> Result<Vec<u8>, String> {
    let bytes = lower(module)?;
    match kind {
        "hex" => Ok(to_hex(&bytes).into_bytes()),
        "raw" => Ok(bytes),
        "asm" => Ok("; ARM64 assembly\n".as_bytes().to_vec()),
        _ => Err(format!("unsupported ARM64 output: {kind}")),
    }
}

fn lower(module: &Module) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();

    for function in &module.functions {
        for instruction in &function.instructions {
            match instruction {
                Instruction::Nop => emit_word(&mut out, 0xD503_201F),
                Instruction::Move { dst, src: Value::Immediate(value) } if *value >= 0 && *value <= 4095 => {
                    if dst.0 > 30 { return Err("ARM64 register out of range".into()); }
                    emit_word(&mut out, 0xD280_0000 | ((*value as u32) << 5) | dst.0);
                }
                Instruction::Add { dst, lhs: Value::Register(lhs), rhs: Value::Register(rhs) } => {
                    emit_word(&mut out, 0x8B00_0000 | (rhs.0 << 16) | (lhs.0 << 5) | dst.0);
                }
                Instruction::Return(_) => emit_word(&mut out, 0xD65F_03C0),
                _ => return Err("ARM64 instruction lowering is not implemented for this instruction".into()),
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
