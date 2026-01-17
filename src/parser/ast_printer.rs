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
      Expr::BinaryOp { operator, left, right } => {
         match operator {
            BinaryOp::Add => {
               println!("{}BinaryOp: Add", indentation);
            },
            BinaryOp::Subtract => {
               println!("{}BinaryOp: Subtract", indentation);
            },
            BinaryOp::Multiply => {
               println!("{}BinaryOp: Multiply", indentation);
            },
            BinaryOp::Divide => {
               println!("{}BinaryOp: Divide", indentation);
            },
            BinaryOp::Modulus => {
               println!("{}BinaryOp: Modulus", indentation);
            },
            BinaryOp::BitwiseOr => {
               println!("{}BinaryOp: BitwiseOr", indentation);
            },
            BinaryOp::BitwiseAnd => {
               println!("{}BinaryOp: BitwiseAnd", indentation);
            },
            BinaryOp::BitwiseXor => {
               println!("{}BinaryOp: BitwiseXor", indentation);
            },
            BinaryOp::LeftShift => {
               println!("{}BinaryOp: LeftShift", indentation);
            },
            BinaryOp::RightShift => {
               println!("{}BinaryOp: RightShift", indentation);
            },
         }
         print_expr(left, indent + 2);
         print_expr(right, indent + 2);
      }
   }
}