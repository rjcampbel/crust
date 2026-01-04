use crate::parser::ast::*;

pub fn print_ast(program: &Program) {
   match program {
      Program::Function(func) => {
         println!("Function: {}", func.name);
         match &func.stmt {
            Stmt::Return(expr) => {
               println!("  Return:");
               match expr {
                  Expr::Integer(value) => {
                     println!("    Integer: {}", value);
                  }
               }
            }
         }
      }
   }
}