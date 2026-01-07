use crate::parser::ast::*;

pub fn print_ast(ast: &AST) {
   println!("AST:");
   match &ast.program {
      Program::Function { name, stmt } => {
         println!("Function: {}", name);
         match stmt {
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