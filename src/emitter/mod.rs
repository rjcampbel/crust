use anyhow::Result;
use crate::codegen::assembly::*;
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn emit_code(assembly_ast: &AssemblyAST, output: &Path) -> Result<()> {
   let mut output = File::create(output)?;
   writeln!(output, "{}", assembly_ast)?;
   Ok(())
}
