use crate::core::{Instruction, Module, Value};

pub fn emit(kind: &str, module: &Module) -> Result<Vec<u8>, String> {
    let bytes = lower(module, false)?;
    match kind {
        "hex" => Ok(to_hex(&bytes).into_bytes()),
        "raw" => Ok(bytes),
        "asm" => Ok("; x86 assembly\n".to_string().into_bytes()),
        _ => Err(format!("unsupported x86 output: {kind}")),
    }
}

fn lower(module: &Module, _long_mode: bool) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();

    for function in &module.functions {
        for instruction in &function.instructions {
            match instruction {
                Instruction::Nop => out.push(0x90),
                Instruction::Move { dst, src: Value::Immediate(value) } => {
                    if dst.0 > 7 || *value < i32::MIN as i64 || *value > i32::MAX as i64 {
                        return Err("x86 immediate is out of range".into());
                    }
                    out.push(0xB8 + dst.0 as u8);
                    out.extend_from_slice(&(*value as i32).to_le_bytes());
                }
                Instruction::Return(_) => out.push(0xC3),
                _ => return Err("x86 instruction lowering is not implemented for this instruction".into()),
            }
        }
    }

    Ok(out)
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}
