use anyhow::{bail, Result};
use crate::codegen::assembly::*;
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn emit_code(program: &Program, output: &Path) -> Result<()> {
   let mut output = File::create(output)?;
   write_program(&program, &mut output)?;
   Ok(())
}

fn write_program(program: &Program, output: &mut File) -> Result<()> {
   match program {
      Program::Function(f) => {
         write_function(&f, output)?;
      }
   }
   Ok(())
}

fn write_function(function: &Function, output: &mut File) -> Result<()> {
   writeln!(output, "\t.globl _{}", function.name)?;
   writeln!(output, "_{}:", function.name)?;
   writeln!(output, "\tpushq\t%rbp")?;
   writeln!(output, "\tmovq\t%rsp, %rbp")?;
   for instr in &function.instructions {
      write_instruction(&instr, output)?;
   }
   Ok(())
}

fn write_instruction(instruction: &Instruction, output: &mut File) -> Result<()> {
   match instruction {
      Instruction::Mov(src, dest) => {
         let src = match src {
            Operand::Immediate(value) => {
               value.to_string()
            }
            _ => {
               bail!("Unsupported source operand for mov instruction")
            }
         };
         let dest = match dest {
            Operand::Register => "%eax".to_string(),
            _ => {
               bail!("Unsupported destination operand for mov instruction")
            }
         };
         writeln!(output, "\tmovl\t{}, {}", src, dest)?;
      }
      Instruction::Return => {
         writeln!(output, "\tmovq\t%rbp, %rsp")?;
         writeln!(output, "\tpopq\t%rbp")?;
         writeln!(output, "\tret")?;
      }
   }
   Ok(())
}