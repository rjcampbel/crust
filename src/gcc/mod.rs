use anyhow::{Result, ensure};
use std::path::Path;
use std::process::Command;

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

pub fn assemble(source: &Path, additional_args: &mut Vec<String>) -> Result<()> {
   let output_ext = if additional_args.contains(&"-c".to_string()) {
      "o"
   } else {
      ""
   };

   let output = source.with_extension(output_ext);
   let mut args: Vec<String> = vec![
         source.to_string_lossy().into_owned(),
         "-o".to_string(),
         output.to_string_lossy().into_owned()
      ];

   args.append(additional_args);
   let output = Command::new("gcc")
      .args(args)
      .output()?;

   ensure!(
      output.status.success(),
      "Error code: {}\n\n{}",
      output.status.code().unwrap_or(-1),
      String::from_utf8_lossy(&output.stderr));

   Ok(())
}