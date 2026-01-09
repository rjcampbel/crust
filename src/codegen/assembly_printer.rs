use crate::codegen::assembly::*;

pub fn print_assembly_ast(assembly_ast: &AssemblyAST) {
   match &assembly_ast.program {
      AssemblyProgram::Function { name, instructions } => {
         println!("Assembly Function: {}", name);
         for instr in instructions {
            match instr {
               Instruction::Mov(src, dest) => {
                  println!("  MOV {:?}, {:?}", src, dest);
               }
               Instruction::Return => {
                  println!("  RETURN");
               },
               _ => {}
            }
         }
      }
   }
}