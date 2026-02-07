use crate::parser::ast::*;

static INDENT_SIZE: usize = 2;

pub fn print_ast(ast: &AST) {
   println!("AST:");
   match &ast.program {
      Program::FunctionDefinition(FunctionDefinition { name, body })  => {
         println!("Function: {}", name);
         print_block(body, INDENT_SIZE);
      }
   }
}

fn print_block(block: &Block, indent: usize) {
   for item in &block.items {
      match item {
         BlockItem::Stmt(s) => print_stmt(&s, indent + 2),
         BlockItem::Decl(decl) => print_decl(decl, indent + INDENT_SIZE),
      }
   }
}

fn print_decl(decl: &Decl, indent: usize) {
   let indentation = " ".repeat(indent);
   match decl {
      Decl::Decl(name, expr, _) => {
         println!("{}Decl: {}", indentation, name);
         if let Some(e) = expr {
            print_expr(&e, indent + INDENT_SIZE);
         }
      }
   }
}

fn print_stmt(stmt: &Stmt, indent: usize) {
   let indentation = " ".repeat(indent);
   match stmt {
      Stmt::Return(expr) => {
         println!("{}Return:", indentation);
         print_expr(expr, indent + INDENT_SIZE);
      },
      Stmt::Expression(expr) => {
         println!("{}Expression:", indentation);
         print_expr(expr, indent + INDENT_SIZE);
      },
      Stmt::Null => {
         println!("{}NULL", indentation);
      },
      Stmt::If(expr, then, else_stmt) => {
         print!("{}If ", indentation);
         print_expr(expr, indent + INDENT_SIZE);
         print_stmt(then, indent + INDENT_SIZE);
         if let Some(stmt) = else_stmt {
            print_stmt(&stmt, indent + INDENT_SIZE);
         }
      },
      Stmt::Compound(block) => {
         print!("{}Compound:", indentation);
         print_block(block, indent + INDENT_SIZE);
      },
      Stmt::Break(_) => {
         println!("{}Break", indentation);
      },
      Stmt::Continue(_) => {
         println!("{}Continue", indentation);
      },
      Stmt::While(expr, body, _) => {
         print!("{}While ", indentation);
         print_expr(expr, indent + INDENT_SIZE);
         print_stmt(body, indent + INDENT_SIZE);
      },
      Stmt::DoWhile(body, expr , _) => {
         print!("{}DoWhile:", indentation);
         print_stmt(body, indent + INDENT_SIZE);
         print!("{}While ", indentation);
         print_expr(expr, indent + INDENT_SIZE);
      },
      Stmt::For(init, condition, increment, body, _) => {
         print!("{}For:", indentation);
         match init {
            ForInit::Decl(decl) => {
               print!("{}Init Decl: ", indentation);
               print_decl(decl, indent + INDENT_SIZE);
            },
            ForInit::Expr(expr) => {
               print!("{}Init Expr: ", indentation);
               if let Some(e) = expr {
                  print_expr(e, indent + INDENT_SIZE);
               }
            }
         }
         if let Some(condition) = condition {
            print!("{}Condition: ", indentation);
            print_expr(condition, indent + INDENT_SIZE);
         }
         if let Some(increment) = increment {
            print!("{}Increment: ", indentation);
            print_expr(increment, indent + INDENT_SIZE);
         }
         print_stmt(body, indent + INDENT_SIZE);
      }
   }
}

fn print_expr(expr: &Expr, indent: usize) {
   let indentation = " ".repeat(indent);
   match expr {
      Expr::Integer(value) => {
         println!("{}Integer: {}", indentation, value);
      },
      Expr::Var(identifier, _) => {
         println!("{}Identifier: {}", indentation, identifier);
      }
      Expr::UnaryOp(operator, expr) => {
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
      Expr::BinaryOp(operator, left, right) => {
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