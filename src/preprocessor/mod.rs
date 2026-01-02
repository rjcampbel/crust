use std::process::Command;
use anyhow::{anyhow, Result};

pub fn preprocess(filename: &str) -> Result<()> {
   let dest = filename.replace(".c", ".i");
   let output = Command::new("gcc")
         .args(["-E", "-P", &filename, "-o", &dest])
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