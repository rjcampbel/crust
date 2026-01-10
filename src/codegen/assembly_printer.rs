use crate::codegen::assembly::*;

pub fn print_assembly_ast(assembly_ast: &AssemblyAST) {
   match &assembly_ast.program {
      AssemblyProgram::Function { name, instructions, .. } => {
         println!("Assembly Function: {}", name);
         for instr in instructions {
            match instr {
               Instruction::Mov(src, dest) => {
                  println!("  MOV {:?}, {:?}", src, dest);
               }
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
               Instruction::AllocateStack(size) => {
                  println!("  ALLOCATE STACK {}", size);
               }
            }
         }
      }
   }
}