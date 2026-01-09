use anyhow::{bail, Result};
use crate::codegen::assembly::*;
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn emit_code(assembly_ast: &AssemblyAST, output: &Path) -> Result<()> {
   let mut output = File::create(output)?;
   write_program(&assembly_ast.program, &mut output)?;
   Ok(())
}

fn write_program(program: &AssemblyProgram, output: &mut File) -> Result<()> {
   match program {
      AssemblyProgram::Function { name, instructions, .. } => {
         write_function(name, instructions, output)?;
      }
   }
   Ok(())
}

fn write_function(name: &String, instructions: &Vec<Instruction>, output: &mut File) -> Result<()> {
   writeln!(output, "\t.globl _{}", name)?;
   writeln!(output, "_{}:", name)?;
   for instr in instructions {
      write_instruction(&instr, output)?;
   }
   Ok(())
}

fn write_instruction(instruction: &Instruction, output: &mut File) -> Result<()> {
   match instruction {
      Instruction::Mov(src, dest) => {
         let src = match src {
            Operand::Immediate(value) => {
               format!("${}", value)
            }
            _ => {
               bail!("Unsupported source operand for mov instruction")
            }
         };
         let dest = match dest {
            Operand::Register(_) => {
               "%eax".to_string()
            },
            _ => {
               bail!("Unsupported destination operand for mov instruction")
            }
         };
         writeln!(output, "\tmovl\t{}, {}", src, dest)?;
      }
      Instruction::Return => {
         writeln!(output, "\tret")?;
      },
      _ => {
         bail!("Unsupported instruction")
      }
   }
   Ok(())
}