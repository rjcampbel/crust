use std::process::Command;
use anyhow::{Result, ensure};
use std::path::Path;

pub fn preprocess(source: &Path, dest: &Path) -> Result<()> {
   let output = Command::new("gcc")
      .args([
         "-E",
         "-P",
         source.to_string_lossy().as_ref(),
         "-o",
         dest.to_string_lossy().as_ref()
      ])
      .output()?;

   ensure!(
      output.status.success(),
      "Error code: {}\n\n{}",
      output.status.code().unwrap_or(-1),
      String::from_utf8_lossy(&output.stderr));

   Ok(())
}

pub fn assemble(source: &Path, output: &Path) -> Result<()> {
   let output = Command::new("gcc")
      .args([
         source.to_string_lossy().as_ref(),
         "-o",
         output.to_string_lossy().as_ref()
      ])
      .output()?;

   ensure!(
      output.status.success(),
      "Error code: {}\n\n{}",
      output.status.code().unwrap_or(-1),
      String::from_utf8_lossy(&output.stderr));

   Ok(())
}