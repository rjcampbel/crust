use crate::parser::ast::*;

pub fn print_ast(ast: &AST) {
   println!("AST:");
   match &ast.program {
      Program::FunctionDefinition(FunctionDefinition::Function(name, body))  => {
         println!("Function: {}", name);
         for item in body {
            match item {
               BlockItem::Stmt(Stmt::Return(expr)) => {
                  println!("  Return:");
                  print_expr(expr, 4);
               },
               BlockItem::Stmt(Stmt::Expression(expr)) => {
                  println!("  Expression:");
                  print_expr(expr, 4);
               },
               BlockItem::Stmt(Stmt::Null) => {
                  println!("  NULL");
               },
               BlockItem::Decl(Decl::Decl(name, expr, _)) => {
                  println!("  Decl: {}", name);
                  if let Some(e) = expr {
                     print_expr(e, 4);
                  }
               }
            }
         }
      }
   }
}

fn print_expr(expr: &Expr, indent: usize) {
   let indentation = " ".repeat(indent);
   match expr {
      Expr::Integer(value, _) => {
         println!("{}Integer: {}", indentation, value);
      },
      Expr::Var(identifier, _) => {
         println!("{}Identifier: {}", indentation, identifier);
      }
      Expr::UnaryOp { operator, expr, .. } => {
         match operator {
            UnaryOp::Complement => {
               println!("{}UnaryOp: Complement", indentation);
            },
            UnaryOp::Negate => {
               println!("{}UnaryOp: Negate", indentation);
            },
            UnaryOp::Not => {
               println!("{}UnaryOp: Not", indentation);
            }
         }
         print_expr(expr, indent + 2);
      },
      Expr::BinaryOp { operator, left, right, .. } => {
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
            BinaryOp::LogicalAnd => {
               println!("{}BinaryOp: LogicalAnd", indentation);
            },
            BinaryOp::LogicalOr => {
               println!("{}BinaryOp: LogicalOr", indentation);
            },
            BinaryOp::Equal => {
               println!("{}BinaryOp: Equal", indentation);
            },
            BinaryOp::NotEqual => {
               println!("{}BinaryOp: NotEqual", indentation);
            },
            BinaryOp::LessThan => {
               println!("{}BinaryOp: LessThan", indentation);
            },
            BinaryOp::LessOrEqual => {
               println!("{}BinaryOp: LessOrEqual", indentation);
            },
            BinaryOp::GreaterThan => {
               println!("{}BinaryOp: GreaterThan", indentation);
            },
            BinaryOp::GreaterOrEqual => {
               println!("{}BinaryOp: GreaterOrEqual", indentation);
            },
         }
         print_expr(left, indent + 2);
         print_expr(right, indent + 2);
      },
      Expr::Assignment(left, right, _) => {
         println!("{}Assignment: ", indent);
         print_expr(left, indent + 2);
         print_expr(right, indent + 2);
      }
   }
}