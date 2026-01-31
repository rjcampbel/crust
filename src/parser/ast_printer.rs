use crate::parser::ast::*;

pub fn print_ast(ast: &AST) {
   println!("AST:");
   match &ast.program {
      Program::FunctionDefinition(FunctionDefinition::Function(name, body))  => {
         println!("Function: {}", name);
         for item in body {
            match item {
               BlockItem::Stmt(s) => print_stmt(s, 0),
               BlockItem::Decl(Decl::Decl(name, expr, _)) => {
                  println!("Decl: {}", name);
                  if let Some(e) = expr {
                     print_expr(e, 4);
                  }
               }
            }
         }
      }
   }
}

fn print_stmt(stmt: &Stmt, indent: usize) {
   let indentation = " ".repeat(indent);
   match stmt {
      Stmt::Return(expr) => {
         println!("{}Return:", indentation);
         print_expr(expr, indent + 4);
      },
      Stmt::Expression(expr) => {
         println!("{}Expression:", indentation);
         print_expr(expr, indent + 4);
      },
      Stmt::Null => {
         println!("{}NULL", indentation);
      },
      Stmt::If(expr, then, else_stmt) => {
         print!("{}If ", indentation);
         print_expr(expr, 0);
         print_stmt(then, indent + 4);
         if let Some(stmt) = else_stmt {
            print_stmt(&stmt, indent + 4);
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
      Expr::UnaryOp(operator, expr, _) => {
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
      Expr::BinaryOp(operator, left, right, _) => {
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
         print_expr(left, indent + 4);
         print_expr(right, indent + 4);
      },
      Expr::Assignment(left, right, _) => {
         println!("{}Assignment: ", indentation);
         print_expr(left, indent + 4);
         print_expr(right, indent + 4);
      },
      Expr::Conditional(condition, true_expr, false_expr) => {
         println!("{}Conditional:", indentation);
         print_expr(condition, indent + 4);
         print_expr(true_expr, indent + 4);
         print_expr(false_expr, indent + 4);
      }
   }
}