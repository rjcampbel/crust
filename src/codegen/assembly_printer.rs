use crate::codegen::assembly::*;

pub fn print_assembly_ast(program: &Program) {
   match program {
      Program::Function(func) => {
         println!("Assembly Function: {}", func.name);
         for instr in &func.instructions {
            match instr {
               Instruction::Mov(src, dest) => {
                  println!("  MOV {:?}, {:?}", src, dest);
               }
               Instruction::Return => {
                  println!("  RETURN");
               }
            }
         }
      }
   }
}