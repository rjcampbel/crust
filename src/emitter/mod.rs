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
   writeln!(output, "\tpushq\t%rbp")?;
   writeln!(output, "\tmovq\t%rsp, %rbp")?;
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
            },
            Operand::Register(Register::AX) => {
               "%eax".to_string()
            },
            Operand::Register(Register::R10D) => {
               "%r10d".to_string()
            },
            Operand::Stack(i) => {
               format!("{}(%rbp)", i)
            },
            _ => {
               bail!("Unsupported source operand for mov instruction")
            }
         };
         let dest = match dest {
            Operand::Register(Register::AX) => {
               "%eax".to_string()
            },
            Operand::Register(Register::R10D) => {
               "%r10d".to_string()
            },
            Operand::Stack(i) => {
               format!("{}(%rbp)", i)
            },
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
      },
      Instruction::AllocateStack(i) => {
         writeln!(output, "\tsubq\t${}, %rsp", i)?;
      },
      Instruction::Unary(op, operand) => {
         let dest = match operand {
            Operand::Register(Register::AX) => {
               "%eax".to_string()
            },
            Operand::Register(Register::R10D) => {
               "%r10d".to_string()
            },
            Operand::Stack(i) => {
               format!("{}(%rbp)", i)
            },
             _ => {
               bail!("Unsupported operand for unary instruction")
            }
         };
         match op {
            UnaryOp::Neg => {
               writeln!(output, "\tnegl\t{}", dest)?;
            },
            UnaryOp::Not => {
               writeln!(output, "\tnotl\t{}", dest)?;
            }
         }
      }
   }
   Ok(())
}