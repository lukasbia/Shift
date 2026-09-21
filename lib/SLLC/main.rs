use std::env;
use std::fs;
use sllc::core::{Function, Instruction, Module, Register, Value};
use sllc::optimization::PassManager;
use sllc::targets::{Target, emit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    let target = argument(&args, "--target")
        .and_then(|v| Target::parse(v))
        .unwrap_or(Target::X86_64);

    let emit_kind = argument(&args, "--emit").unwrap_or("hex");
    let output = argument(&args, "--output").unwrap_or("a.sll");

    let mut module = demo_module();

    let level = argument(&args, "--opt-level")
        .or_else(|| argument(&args, "-O"))
        .unwrap_or("2");

    PassManager::for_level(level).run(&mut module);

    match emit(&target, emit_kind, &module) {
        Ok(bytes) => {
            if emit_kind == "hex" || emit_kind == "asm" || emit_kind == "wasm" {
                println!("{}", String::from_utf8_lossy(&bytes));
            } else if let Err(error) = fs::write(output, bytes) {
                eprintln!("sllc: {error}");
            }
        }
        Err(error) => eprintln!("sllc: {error}"),
    }
}

fn argument<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].as_str())
}

fn demo_module() -> Module {
    Module {
        functions: vec![
            Function {
                name: "main".into(),
                instructions: vec![
                    Instruction::Move {
                        dst: Register(0),
                        src: Value::Immediate(10),
                    },
                    Instruction::Move {
                        dst: Register(1),
                        src: Value::Immediate(20),
                    },
                    Instruction::Add {
                        dst: Register(2),
                        lhs: Value::Register(Register(0)),
                        rhs: Value::Register(Register(1)),
                    },
                    Instruction::Return(Value::Register(Register(2))),
                ],
            }
        ],
    }
}

fn print_help() {
    println!("sllc - Shift Low Level Compiler");
    println!();
    println!("  --target x86|x86_64|arm32|arm64|wasm32");
    println!("  --emit hex|raw|asm|wasm");
    println!("  --output <file>");
    println!("  -O <0|1|2|3|s>");
}
