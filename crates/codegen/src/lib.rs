use std::path::Path;
use std::process::{Command, ExitStatus};

mod code_generator;
mod error;

pub use code_generator::CodeGenerator;
pub use error::CodegenError;

pub fn link(obj_file: &Path, output: &Path) -> Result<ExitStatus, CodegenError> {
    Command::new("cc")
        .args([&obj_file, Path::new("-o"), output])
        .status()
        .map_err(|_| CodegenError::LinkError)
}
