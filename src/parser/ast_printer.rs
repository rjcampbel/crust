use crate::parser::ast::*;

pub fn print_ast(program: &Program) {
   match program {
      Program::Function(func) => {
         println!("Function: {}", func.name);
         match &func.stmt {
            Stmt::Return(expr) => {
               println!("  Return:");
               print_expr(expr, 4);
            }
         }
      }
   }
}

fn print_expr(expr: &Expr, indent: usize) {
   let indentation = " ".repeat(indent);
   match expr {
      Expr::Integer(value) => {
         println!("{}Integer: {}", indentation, value);
      },
      Expr::UnaryOp { operator, expr } => {
         match operator {
            UnaryOp::Complement => {
               println!("{}UnaryOp: Complement", indentation);
            },
            UnaryOp::Negate => {
               println!("{}UnaryOp: Negate", indentation);
            },
         }
         print_expr(expr, indent + 2);
      },
   }
}