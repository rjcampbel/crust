pub mod assembly;
mod assembly_printer;
mod stack_allocator;

use crate::tacky::{self, tacky::*};
use anyhow::Result;
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
            let ret = generate_operand(val);
            instructions.push(Instruction::Mov(ret, Operand::Register(Register::AX)));
            instructions.push(Instruction::Return);
         },
         Instr::Unary { operator, src, dest } => {
            let src = generate_operand(src);
            let dst = generate_operand(dest);
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
         Instr::Binary { operator, left, right, dest } => {
            let left = generate_operand(left);
            let right = generate_operand(right);
            let dst = generate_operand(dest);
            match operator {
               tacky::tacky::BinaryOp::Add => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Add, right.clone(), dst.clone()));
               },
               tacky::tacky::BinaryOp::Subtract => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Sub, right.clone(), dst.clone()));
               },
               tacky::tacky::BinaryOp::Multiply => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Mult, right.clone(), dst.clone()));
               },
               &tacky::tacky::BinaryOp::Divide => {
                  instructions.push(Instruction::Mov(left, Operand::Register(Register::AX)));
                  instructions.push(Instruction::Cdq);
                  instructions.push(Instruction::Idiv(right));
                  instructions.push(Instruction::Mov(Operand::Register(Register::AX), dst));
               },
               &tacky::tacky::BinaryOp::Modulus => {
                  instructions.push(Instruction::Mov(left, Operand::Register(Register::AX)));
                  instructions.push(Instruction::Cdq);
                  instructions.push(Instruction::Idiv(right));
                  instructions.push(Instruction::Mov(Operand::Register(Register::DX), dst));
               },
               &tacky::tacky::BinaryOp::BitwiseAnd => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseAnd, right.clone(), dst.clone()));
               },
               &tacky::tacky::BinaryOp::BitwiseOr => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseOr, right.clone(), dst.clone()));
               },
               &tacky::tacky::BinaryOp::BitwiseXor => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseXor, right.clone(), dst.clone()));
               },
               &tacky::tacky::BinaryOp::LeftShift => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Shl(right.clone(), dst.clone()));
               },
               &tacky::tacky::BinaryOp::RightShift => {
                  instructions.push(Instruction::Mov(left.clone(), dst.clone()));
                  instructions.push(Instruction::Shr(right.clone(), dst.clone()));
               },
            };
         }
      }
   }
   Ok(instructions)
}

fn generate_operand(val: &Val) -> Operand {
   match val {
      Val::Integer(i) => Operand::Immediate(*i),
      Val::Var(name) => Operand::Pseudo(name.clone()),
   }
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
               Instruction::Binary(_, left, right) => {
                  match left {
                     Operand::Pseudo(s) => {
                        *left = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
                  match right {
                     Operand::Pseudo(s) => {
                        *right = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
               },
               Instruction::Idiv(operand) => {
                  match operand {
                     Operand::Pseudo(s) => {
                        *operand = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
               },
               Instruction::Shl(dest, count) => {
                  match dest {
                     Operand::Pseudo(s) => {
                        *dest = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
                  match count {
                     Operand::Pseudo(s) => {
                        *count = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
               },
               Instruction::Shr(dest, count) => {
                  match dest {
                     Operand::Pseudo(s) => {
                        *dest = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
                     },
                     _ => {}
                  }
                  match count {
                     Operand::Pseudo(s) => {
                        *count = Operand::Stack(stack_allocator.allocate(s.to_string(), 4));
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
         let mut new_instructions = Vec::new();
         new_instructions.push(Instruction::AllocateStack(stack_size));

         for instr in instructions.iter() {
            match instr {
               Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
                  let src_val = *src;
                  let dst_val = *dst;
                  let instr = Instruction::Mov(Operand::Stack(src_val), Operand::Register(Register::R10));
                  new_instructions.push(instr);
                  let new_instr = Instruction::Mov(Operand::Register(Register::R10), Operand::Stack(dst_val));
                  new_instructions.push(new_instr);
               },
               Instruction::Idiv(Operand::Immediate(i)) => {
                  let imm = *i;
                  let instr = Instruction::Mov(Operand::Immediate(imm), Operand::Register(Register::R10));
                  new_instructions.push(instr);
                  let new_instr = Instruction::Idiv(Operand::Register(Register::R10));
                  new_instructions.push(new_instr);
               }
               Instruction::Binary(op @ (assembly::BinaryOp::Add | assembly::BinaryOp::Sub | assembly::BinaryOp::BitwiseAnd | assembly::BinaryOp::BitwiseOr | assembly::BinaryOp::BitwiseXor), Operand::Stack(src), Operand::Stack(dst)) => {
                  let src_val = *src;
                  let dst_val = *dst;
                  let op = op.clone();
                  let instr = Instruction::Mov(Operand::Stack(src_val), Operand::Register(Register::R10));
                  new_instructions.push(instr);
                  let new_instr = Instruction::Binary(
                     op,
                     Operand::Register(Register::R10),
                     Operand::Stack(dst_val)
                  );
                  new_instructions.push(new_instr);
               },
               Instruction::Binary(assembly::BinaryOp::Mult, src @ _, dst @ Operand::Stack(_)) => {
                  let instr1 = Instruction::Mov(dst.clone(), Operand::Register(Register::R11));
                  let instr2 = Instruction::Binary(
                     assembly::BinaryOp::Mult,
                     src.clone(),
                     Operand::Register(Register::R11)
                  );
                  let instr3 = Instruction::Mov(
                     Operand::Register(Register::R11),
                     dst.clone()
                  );
                  new_instructions.push(instr1);
                  new_instructions.push(instr2);
                  new_instructions.push(instr3);
               },
               Instruction::Shl(count @ Operand::Stack(_), dest @ _) => {
                  new_instructions.push(Instruction::Movb(count.clone(), Operand::Register(Register::CL)));
                  new_instructions.push(Instruction::Shl(Operand::Register(Register::CL), dest.clone()));
               },
               Instruction::Shr(count @ Operand::Stack(_), dest @ _) => {
                  new_instructions.push(Instruction::Movb(count.clone(), Operand::Register(Register::CL)));
                  new_instructions.push(Instruction::Shr(Operand::Register(Register::CL), dest.clone()));
               },
               i @ _ => {
                  new_instructions.push(i.clone());
               }
            }
         }

         *instructions = new_instructions;
      }
   }
}