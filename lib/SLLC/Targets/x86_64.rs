use crate::core::{Instruction, Module, Value};

pub fn emit(kind: &str, module: &Module) -> Result<Vec<u8>, String> {
    let bytes = lower(module)?;
    match kind {
        "hex" => Ok(to_hex(&bytes).into_bytes()),
        "raw" => Ok(bytes),
        "asm" => Ok(assembly(module).into_bytes()),
        _ => Err(format!("unsupported x86_64 output: {kind}")),
    }
}

fn lower(module: &Module) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();

    for function in &module.functions {
        for instruction in &function.instructions {
            match instruction {
                Instruction::Nop => out.push(0x90),
                Instruction::Move { dst, src: Value::Immediate(value) } => {
                    if dst.0 > 7 {
                        return Err("x86_64 demo backend supports registers r0-r7".into());
                    }
                    out.extend_from_slice(&[0x48, 0xB8 + dst.0 as u8]);
                    out.extend_from_slice(&value.to_le_bytes());
                }
                Instruction::Add { dst, lhs, rhs } => {
                    let (lhs, rhs) = match (lhs, rhs) {
                        (Value::Register(a), Value::Register(b)) => (a.0, b.0),
                        _ => return Err("x86_64 add requires registers".into()),
                    };
                    if dst.0 > 7 || lhs > 7 || rhs > 7 {
                        return Err("x86_64 register out of range".into());
                    }
                    if dst.0 != lhs {
                        out.extend_from_slice(&[0x48, 0x89, 0xC0 | ((lhs as u8) << 3) | dst.0 as u8]);
                    }
                    out.extend_from_slice(&[0x48, 0x01, 0xC0 | ((rhs as u8) << 3) | dst.0 as u8]);
                }
                Instruction::Return(_) => out.push(0xC3),
                _ => return Err("x86_64 instruction lowering is not implemented for this instruction".into()),
            }
        }
    }

    Ok(out)
}

fn assembly(module: &Module) -> String {
    let mut text = String::from("; SLLC x86_64 assembly\n");
    for function in &module.functions {
        text.push_str(&format!("{}:\n", function.name));
        for instruction in &function.instructions {
            text.push_str(&format!("  {:?}\n", instruction));
        }
    }
    text
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}
