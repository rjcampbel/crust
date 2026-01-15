pub mod assembly;
mod assembly_printer;
mod stack_allocator;

use crate::tacky::{self, tacky::*};
use anyhow::{bail,Result};
use assembly::*;
use assembly_printer::print_assembly_ast;
use stack_allocator::StackAllocator;

pub fn codegen(tacky: &TackyAST, print_assembly: bool) -> Result<AssemblyAST> {
   let assembly_ast = generate_assembly(&tacky)?;
   if print_assembly {
      print_assembly_ast(&assembly_ast);
   }
   Ok(assembly_ast)
}

fn generate_assembly(tacky: &TackyAST) -> Result<AssemblyAST> {
   let mut assembly = match &tacky.program {
      TackyProgram::Function { identifier, body }=> {
         let function = generate_function(&identifier, &body)?;
         Ok(AssemblyAST { program: function })
      }
   };

   if let Ok(ref mut ass) = assembly {
      replace_pseudoregisters(ass);
      fixup_instructions(ass);
   }
   assembly
}

fn generate_function(name: &String, instrs: &Vec<Instr>) -> Result<AssemblyProgram> {
   let instructions = generate_instructions(instrs)?;
   let assembly_function = AssemblyProgram::Function {
      name: name.clone(),
      instructions,
      stack_allocator: StackAllocator::new(),
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
         },
         _ => {
            bail!("Unsupported instruction in codegen");
         }
      }
   }
   Ok(instructions)
}

fn replace_pseudoregisters(assembly: &mut AssemblyAST) {
   match &mut assembly.program {
      AssemblyProgram::Function { instructions, stack_allocator , .. } => {
         for instr in instructions {
            match instr {
               Instruction::Mov(src, dst) => {
                  match src {
                     Operand::Pseudo(s) => {
                        *src = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  };
                  match dst {
                     Operand::Pseudo(s) => {
                        *dst = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  };
               },
               Instruction::Unary(_, operand) => {
                  match operand {
                     Operand::Pseudo(s) => {
                        *operand = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
               },
               _ => {}
            }
         }
      }
   }
}

fn fixup_instructions(assembly: &mut AssemblyAST) {
   match &mut assembly.program {
      AssemblyProgram::Function { instructions, stack_allocator , .. } => {
         let stack_size = stack_allocator.get();
         instructions.insert(0, Instruction::AllocateStack(stack_size));

         let mut insertions = Vec::new();
         for (index, instr) in instructions.iter_mut().enumerate(){
            match instr {
               Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
                  let src_val = *src;
                  let dst_val = *dst;
                  *instr = Instruction::Mov(Operand::Stack(src_val), Operand::Register(Register::R10D));
                  let new_instr = Instruction::Mov(Operand::Register(Register::R10D), Operand::Stack(dst_val));
                  insertions.push((index + 1, new_instr));
               },
               _ => {}
            }
         }

         for (index, new_instr) in insertions.into_iter().rev() {
            instructions.insert(index, new_instr);
         }
      }
   }
}