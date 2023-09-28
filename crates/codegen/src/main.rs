use std::{path::Path, process::Command};
use ::codegen::CodeGenerator;

fn main() {
    let mut code_generator = CodeGenerator::new();

    code_generator.generate_main();

    let code = code_generator.emit();
    std::fs::write("main.o", code).unwrap();

    link(Path::new("main.o"), Path::new("main"));
}

fn link(obj_file: &Path, output: &Path) {
    Command::new("cc")
        .args(&[&obj_file, Path::new("-o"), output])
        .status()
        .unwrap();
}
