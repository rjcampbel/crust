use crate::tacky::tacky::*;

pub fn print_tacky_ast(tacky_ast: &TackyAST) {
   println!("Tacky AST:");
   match &tacky_ast.program {
      TackyProgram::Function { identifier, body } => {
         println!("Tacky Function: {}", identifier);
         for instr in body {
            match instr {
               Instr::Return(val) => {
                  match val {
                     Val::Integer(i) => println!("  RETURN {}", i),
                     Val::Var(v) => println!("  RETURN {}", v),
                  }
               },
               Instr::Unary { operator, src, dest } => {
                  println!("  {:?} {:?} -> {:?}", operator, src, dest);
               },
               Instr::Binary { operator, left, right, dest } => {
                  println!("  {:?} {:?}, {:?} -> {:?}", operator, left, right, dest);
               }
            }
         }
      }
   }
}