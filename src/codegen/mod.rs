pub mod assembly;
mod assembly_printer;
mod stack_allocator;

use crate::codegen::assembly::*;
use crate::tacky::tacky::{BinaryOp, Instr, UnaryOp, TackyAST, TackyProgram, Val};

use anyhow::Result;
use assembly_printer::print_assembly_ast;
use stack_allocator::StackAllocator;

pub fn codegen(tacky: TackyAST, print_assembly: bool) -> Result<AssemblyAST> {
   let assembly_ast = generate_assembly(tacky)?;
   if print_assembly {
      print_assembly_ast(&assembly_ast);
   }
   Ok(assembly_ast)
}

fn generate_assembly(tacky: TackyAST) -> Result<AssemblyAST> {
   let mut assembly = match tacky.program {
      TackyProgram::Function(identifier, body)=> {
         let function = generate_function(identifier, body)?;
         Ok(AssemblyAST { program: function })
      }
   };

   if let Ok(ref mut ass) = assembly {
      replace_pseudoregisters(ass);
      fixup_instructions(ass);
   }
   assembly
}

fn generate_function(name: String, instrs: Vec<Instr>) -> Result<AssemblyProgram> {
   let instructions = generate_instructions(instrs)?;
   let assembly_function = AssemblyProgram::Function(name, instructions, StackAllocator::new());
   Ok(assembly_function)
}

fn generate_instructions(instrs: Vec<Instr>) -> Result<Vec<Instruction>> {
   let mut instructions = Vec::new();
   for instr in instrs {
      match instr {
         Instr::Return(val) => {
            let ret = generate_operand(val);
            instructions.push(Instruction::Mov(ret, Operand::Register(Register::AX)));
            instructions.push(Instruction::Return);
         },
         Instr::Unary(UnaryOp::Not, src, dest) => {
            instructions.push(Instruction::Cmp(Operand::Immediate(0), generate_operand(src)));
            instructions.push(Instruction::Mov(Operand::Immediate(0), generate_operand(dest.clone())));
            instructions.push(Instruction::SetCC(ConditionCode::E, generate_operand(dest)));
         },
         Instr::Unary(operator, src, dest) => {
            let src = generate_operand(src);
            let dst = generate_operand(dest);
            let op = match operator {
               UnaryOp::Negate => {
                  assembly::UnaryOp::Neg
               },
               UnaryOp::Complement => {
                  assembly::UnaryOp::Not
               },
               _ => unreachable!()
            };
            instructions.push(Instruction::Mov(src, dst.clone()));
            instructions.push(Instruction::Unary(op, dst));
         },
         Instr::Binary(operator @ (BinaryOp::Equal | BinaryOp::NotEqual | BinaryOp::LessThan | BinaryOp::LessOrEqual | BinaryOp::GreaterThan | BinaryOp::GreaterOrEqual), left, right, dest) => {
            instructions.push(Instruction::Cmp(generate_operand(right), generate_operand(left)));
            instructions.push(Instruction::Mov(Operand::Immediate(0), generate_operand(dest.clone())));
            let code = match operator {
               BinaryOp::Equal => ConditionCode::E,
               BinaryOp::NotEqual => ConditionCode::NE,
               BinaryOp::LessThan => ConditionCode::L,
               BinaryOp::LessOrEqual => ConditionCode::LE,
               BinaryOp::GreaterThan => ConditionCode::G,
               BinaryOp::GreaterOrEqual => ConditionCode::GE,
               _ => unreachable!()
            };
            instructions.push(Instruction::SetCC(code, generate_operand(dest)));
         },
         Instr::Binary(operator, left, right, dest) => {
            let left = generate_operand(left);
            let right = generate_operand(right);
            let dst = generate_operand(dest);
            match operator {
               BinaryOp::Add => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Add, right, dst));
               },
               BinaryOp::Subtract => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Sub, right, dst));
               },
               BinaryOp::Multiply => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::Mult, right, dst));
               },
               BinaryOp::Divide => {
                  instructions.push(Instruction::Mov(left, Operand::Register(Register::AX)));
                  instructions.push(Instruction::Cdq);
                  instructions.push(Instruction::Idiv(right));
                  instructions.push(Instruction::Mov(Operand::Register(Register::AX), dst));
               },
               BinaryOp::Modulus => {
                  instructions.push(Instruction::Mov(left, Operand::Register(Register::AX)));
                  instructions.push(Instruction::Cdq);
                  instructions.push(Instruction::Idiv(right));
                  instructions.push(Instruction::Mov(Operand::Register(Register::DX), dst));
               },
               BinaryOp::BitwiseAnd => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseAnd, right, dst));
               },
               BinaryOp::BitwiseOr => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseOr, right, dst));
               },
               BinaryOp::BitwiseXor => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Binary(assembly::BinaryOp::BitwiseXor, right, dst));
               },
               BinaryOp::LeftShift => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Shl(right, dst));
               },
               BinaryOp::RightShift => {
                  instructions.push(Instruction::Mov(left, dst.clone()));
                  instructions.push(Instruction::Shr(right, dst));
               },
               _ => unreachable!()
            };
         },
         Instr::Jump(label) => {
            instructions.push(Instruction::Jmp(label));
         },
         Instr::JumpIfNotZero(condition, target) => {
            instructions.push(Instruction::Cmp(Operand::Immediate(0), generate_operand(condition)));
            instructions.push(Instruction::JmpCC(ConditionCode::NE, target));
         },
         Instr::JumpIfZero(condition, target) => {
            instructions.push(Instruction::Cmp(Operand::Immediate(0), generate_operand(condition)));
            instructions.push(Instruction::JmpCC(ConditionCode::E, target));
         },
         Instr::Copy(src, dest) => {
            instructions.push(Instruction::Mov(generate_operand(src), generate_operand(dest)));
         },
         Instr::Label(label) => {
            instructions.push(Instruction::Label(label));
         }
      }
   }
   Ok(instructions)
}

fn generate_operand(val: Val) -> Operand {
   match val {
      Val::Integer(i) => Operand::Immediate(i),
      Val::Var(name) => Operand::Pseudo(name),
   }
}

fn replace_pseudoregisters(assembly: &mut AssemblyAST) {
   match &mut assembly.program {
      AssemblyProgram::Function(_, instructions, stack_allocator) => {
         for instr in instructions {
            match instr {
               Instruction::Mov(src, dst) => {
                  convert_pseudo_stack(src, 4, stack_allocator);
                  convert_pseudo_stack(dst, 4, stack_allocator);
               },
               Instruction::Unary(_, operand) => {
                  convert_pseudo_stack(operand, 4, stack_allocator);
               },
               Instruction::Binary(_, left, right) => {
                  convert_pseudo_stack(left, 4, stack_allocator);
                  convert_pseudo_stack(right, 4, stack_allocator);
               },
               Instruction::Idiv(operand) => {
                  convert_pseudo_stack(operand, 4, stack_allocator);
               },
               Instruction::Shl(dest, count) => {
                  convert_pseudo_stack(dest, 4, stack_allocator);
                  convert_pseudo_stack(count, 4, stack_allocator);
               },
               Instruction::Shr(dest, count) => {
                  convert_pseudo_stack(dest, 4, stack_allocator);
                  convert_pseudo_stack(count, 4, stack_allocator);
               },
               Instruction::Cmp(left, right) => {
                  convert_pseudo_stack(left, 4, stack_allocator);
                  convert_pseudo_stack(right, 4, stack_allocator);
               },
               Instruction::SetCC(_, operand) => {
                  convert_pseudo_stack(operand, 4, stack_allocator);
               },
               _ => {}
            }
         }
      }
   }
}

fn fixup_instructions(assembly: &mut AssemblyAST) {
   match &mut assembly.program {
      AssemblyProgram::Function(_, instructions, stack_allocator) => {
         let stack_size = stack_allocator.get();
         let mut new_instructions = Vec::new();
         new_instructions.push(Instruction::AllocateStack(stack_size));

         for instr in instructions.iter() {
            match instr {
               Instruction::Mov(Operand::Stack(src), Operand::Stack(dst)) => {
                  new_instructions.push(Instruction::Mov(Operand::Stack(*src), Operand::Register(Register::R10)));
                  new_instructions.push(Instruction::Mov(Operand::Register(Register::R10), Operand::Stack(*dst)));
               },
               Instruction::Idiv(Operand::Immediate(i)) => {
                  new_instructions.push(Instruction::Mov(Operand::Immediate(*i), Operand::Register(Register::R10)));
                  new_instructions.push(Instruction::Idiv(Operand::Register(Register::R10)));
               }
               Instruction::Binary(op @ (assembly::BinaryOp::Add | assembly::BinaryOp::Sub | assembly::BinaryOp::BitwiseAnd | assembly::BinaryOp::BitwiseOr | assembly::BinaryOp::BitwiseXor), Operand::Stack(src), Operand::Stack(dst)) => {
                  new_instructions.push(Instruction::Mov(Operand::Stack(*src), Operand::Register(Register::R10)));
                  new_instructions.push(Instruction::Binary(op.clone(), Operand::Register(Register::R10), Operand::Stack(*dst)));
               },
               Instruction::Binary(assembly::BinaryOp::Mult, src @ _, dst @ Operand::Stack(_)) => {
                  new_instructions.push(Instruction::Mov(dst.clone(), Operand::Register(Register::R11)));
                  new_instructions.push(Instruction::Binary(assembly::BinaryOp::Mult, src.clone(), Operand::Register(Register::R11)));
                  new_instructions.push(Instruction::Mov(Operand::Register(Register::R11), dst.clone()));
               },
               Instruction::Shl(count @ Operand::Stack(_), dest @ _) => {
                  new_instructions.push(Instruction::Movb(count.clone(), Operand::Register(Register::CL)));
                  new_instructions.push(Instruction::Shl(Operand::Register(Register::CL), dest.clone()));
               },
               Instruction::Shr(count @ Operand::Stack(_), dest @ _) => {
                  new_instructions.push(Instruction::Movb(count.clone(), Operand::Register(Register::CL)));
                  new_instructions.push(Instruction::Shr(Operand::Register(Register::CL), dest.clone()));
               },
               Instruction::Cmp(Operand::Stack(left), Operand::Stack(right)) => {
                  new_instructions.push(Instruction::Mov(Operand::Stack(*left), Operand::Register(Register::R10)));
                  new_instructions.push(Instruction::Cmp(Operand::Register(Register::R10), Operand::Stack(*right)));
               },
               Instruction::Cmp(left @ _, right @ Operand::Immediate(_)) => {
                  new_instructions.push(Instruction::Mov(right.clone(), Operand::Register(Register::R10)));
                  new_instructions.push(Instruction::Cmp(left.clone(), Operand::Register(Register::R10)));
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

fn convert_pseudo_stack(pseudo: &mut Operand, size: i64, stack_allocator: &mut StackAllocator) {
   if let Operand::Pseudo(name) = pseudo {
      *pseudo = Operand::Stack(stack_allocator.allocate(name.to_string(), size));
   };
}