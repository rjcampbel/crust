use crate::codegen::assembly::*;

use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn emit_code(assembly_ast: &Assembly, output: &Path) -> Result<()> {
   let mut output = File::create(output)?;
   writeln!(output, "{}", assembly_ast)?;
   Ok(())
}
