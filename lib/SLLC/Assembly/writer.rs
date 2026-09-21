use crate::core::Module;

pub struct AssemblyWriter;

impl AssemblyWriter {
    pub fn write(module: &Module) -> String {
        let mut output = String::new();

        for function in &module.functions {
            output.push_str(&format!("{}:\n", function.name));
            for instruction in &function.instructions {
                output.push_str(&format!("    {:?}\n", instruction));
            }
        }

        output
    }
}
