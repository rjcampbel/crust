use std::process::Command;
use anyhow::{anyhow, Result};
use std::path::Path;

pub fn preprocess(source: &Path, dest: &Path) -> Result<()> {
   let output = Command::new("gcc")
         .args(["-E", "-P", source.to_string_lossy().as_ref(), "-o", dest.to_string_lossy().as_ref()])
         .output()?;

   if !output.status.success() {
      return Err(anyhow!(format!(
         "Error code: {}\n\n{}",
         output.status.code().unwrap_or(-1),
         String::from_utf8_lossy(&output.stderr)
      )));
   }

   Ok(())
}