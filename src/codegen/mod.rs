pub mod assembly;
mod assembly_printer;

use crate::tacky::{self, tacky::*};
use anyhow::Result;
use assembly::*;
use assembly_printer::print_assembly_ast;

pub fn codegen(tacky: &TackyAST, print_assembly: bool) -> Result<AssemblyAST> {
   let assembly_ast = generate_assembly(&tacky)?;
   if print_assembly {
      print_assembly_ast(&assembly_ast);
   }
   Ok(assembly_ast)
}

fn generate_assembly(tacky: &TackyAST) -> Result<AssemblyAST> {
   match &tacky.program {
      TackyProgram::Function { identifier, body }=> {
         let function = generate_function(&identifier, &body)?;
         Ok(AssemblyAST { program: function })
      }
   }
}

fn generate_function(name: &String, instrs: &Vec<Instr>) -> Result<AssemblyProgram> {
   let instructions = generate_instructions(instrs)?;
   let assembly_function = AssemblyProgram::Function {
      name: name.clone(),
      instructions,
   };
   Ok(assembly_function)
}

fn generate_instructions(instrs: &Vec<Instr>) -> Result<Vec<Instruction>> {
   let mut instructions = Vec::new();
   for instr in instrs {
      match instr {
         Instr::Return(val) => {
            match val {
               Val::Integer(value) => {
                  instructions.push(Instruction::Mov(Operand::Immediate(*value), Operand::Register(Register::AX)));
               },
               Val::Var(name) => {
                  instructions.push(Instruction::Mov(Operand::Pseudo(name.clone()), Operand::Register(Register::AX)));
               }
            }
            instructions.push(Instruction::Return);
         },
         Instr::Unary { operator, src, dest } => {
            let src = match src {
               Val::Integer(i) => {
                  Operand::Immediate(*i)
               },
               Val::Var(i) => {
                  Operand::Pseudo(i.clone())
               }
            };
            let dst = match dest {
               Val::Integer(i) => {
                  Operand::Immediate(*i)
               },
               Val::Var(i) => {
                  Operand::Pseudo(i.clone())
               }
            };
            let op = match operator {
               tacky::tacky::UnaryOp::Negate => {
                  assembly::UnaryOp::Neg
               },
               tacky::tacky::UnaryOp::Complement => {
                  assembly::UnaryOp::Not
               }
            };
            instructions.push(Instruction::Mov(src, dst.clone()));
            instructions.push(Instruction::Unary(op, dst));
         }
      }
   }
   Ok(instructions)
}
