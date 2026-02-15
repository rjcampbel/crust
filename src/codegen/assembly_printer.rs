use super::assembly::*;

pub fn print_assembly(assembly: &Assembly) {
   match &assembly.program {
      AssemblyProgram::Function(name, instructions, ..) => {
         println!("Assembly Function: {}", name);
         for instr in instructions {
            match instr {
               Instruction::Mov(src, dest) => {
                  println!("  MOV {:?}, {:?}", src, dest);
               },
               Instruction::Movb(src, dest) => {
                  println!("  MOVB {:?}, {:?}", src, dest);
               },
               Instruction::Return => {
                  println!("  RETURN");
               },
               Instruction::Unary(operator, operand ) => {
                  match operator {
                     UnaryOp::Neg => {
                        println!("  NEG {:?}", operand);
                     },
                     UnaryOp::Not => {
                        println!("  NOT {:?}", operand);
                     }
                  }
               },
               Instruction::Binary(op, left, right) => {
                  match op {
                     BinaryOp::Add => {
                        println!("  ADD {:?}, {:?}", left, right);
                     },
                     BinaryOp::Sub => {
                        println!("  SUB {:?}, {:?}", left, right);
                     },
                     BinaryOp::Mult => {
                        println!("  MUL {:?}, {:?}", left, right);
                     },
                     BinaryOp::BitwiseAnd => {
                        println!("  BITAND {:?}, {:?}", left, right);
                     },
                     BinaryOp::BitwiseOr => {
                        println!("  BITOR {:?}, {:?}", left, right);
                     },
                     BinaryOp::BitwiseXor => {
                        println!("  XOR {:?}, {:?}", left, right);
                     },
                  }
               },
               Instruction::Cmp(left, right) => {
                  println!("  CMP {:?}, {:?}", left, right);
               }
               Instruction::Shl(src, dest) => {
                  println!("  SHL {:?}, {:?}", src, dest);
               },
               Instruction::Shr(src, dest) => {
                  println!("  SHR {:?}, {:?}", src, dest);
               },
               Instruction::Idiv(operand) => {
                  println!("  IDIV {:?}", operand);
               },
               Instruction::Cdq => {
                  println!("  CDQ");
               },
               Instruction::Jmp(dest) => {
                  println!("  JUMP {:?}", dest);
               },
               Instruction::JmpCC(code, dest, ) => {
                  println!("  JUMPCC {:?}, {:?}", code, dest);
               },
               Instruction::SetCC(code, dest) => {
                  println!("  SETCC {:?}, {:?}" , code, dest);
               },
               Instruction::Label(lbl) => {
                  println!("  LBL {:?}", lbl);
               }
               Instruction::AllocateStack(size) => {
                  println!("  ALLOCATE STACK {}", size);
               }
            }
         }
      }
   }
}