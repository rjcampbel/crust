use crate::error;
use crate::name_generator;
use crate::parser::ast_printer;
use crate::parser::ast::*;

use anyhow::{Result, bail};
use std::collections::HashMap;
use std::rc::Rc;

struct Validator {
   variable_map: HashMap<Rc<String>, Rc<String>>
}

pub fn validate(ast: &mut AST, print_ast: bool) -> Result<()> {
   let mut validator = Validator::new();
   validator.validate(ast)?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(())
}

impl Validator {
   fn new() -> Self {
      Self {
         variable_map: HashMap::new()
      }
   }

   fn validate(&mut self, ast: &mut AST) -> Result<()> {
      self.resolve_program(&mut ast.program)?;
      Ok(())
   }

   fn resolve_program(&mut self, program: &mut Program) -> Result<()> {
      match program {
         Program::FunctionDefinition(FunctionDefinition::Function(_, body)) => {
            self.resolve_function(body)?;
            Ok(())
         }
      }
   }

   fn resolve_function(&mut self, body: &mut Vec<BlockItem>) -> Result<()> {
      for block_item in &mut *body {
         self.validate_block_item(block_item)?;
      }
      Ok(())
   }

   fn validate_block_item(&mut self, item: &BlockItem) -> Result<BlockItem> {
      match item {
         BlockItem::Stmt(stmt) => {
            Ok(BlockItem::Stmt(self.resolve_statement(stmt)?))
         },
         BlockItem::Decl(decl) => {
            Ok(BlockItem::Decl(self.resolve_declaration(decl)?))
         }
      }
   }

   fn resolve_statement(&mut self, stmt: &Stmt) -> Result<Stmt> {
      match stmt {
         Stmt::Expression(e) => {
            Ok(Stmt::Expression(self.resolve_expr(&e)?))
         },
         Stmt::Return(e) => {
            Ok(Stmt::Return(self.resolve_expr(&e)?))
         },
         Stmt::Null => Ok(Stmt::Null),
         Stmt::If(expr, then_stmt, else_stmt) => {
            let else_stmt = if let Some(s) = else_stmt {
               Some(Box::new(self.resolve_statement(s)?))
            } else {
               None
            };
            Ok(Stmt::If(self.resolve_expr(expr)?, Box::new(self.resolve_statement(then_stmt)?), else_stmt))
         }
      }
   }

   fn resolve_declaration(&mut self, decl: &Decl) -> Result<Decl> {
      let Decl::Decl(name, initializer, line_number) = decl;
      if self.variable_map.contains_key(name) {
         bail!(error::error(*line_number, format!("\"{}\" already declared.", name), error::ErrorType::SemanticError))
      }
      let unique_name = Rc::new(name_generator::uniquify_identifier(name));
      self.variable_map.insert(Rc::clone(&name), Rc::clone(&unique_name));

      let initializer = if let Some(expr) = initializer {
         Some(self.resolve_expr(expr)?)
      } else {
         None
      };
      Ok(Decl::Decl(unique_name, initializer, *line_number))
   }

   fn resolve_expr(&mut self, expr: &Expr) -> Result<Expr> {
      match expr {
         Expr::Assignment(left, right, line_number) => {
            if let Expr::Var(_, _) = **left {
               Ok(Expr::Assignment(Box::new(self.resolve_expr(&**left)?), Box::new(self.resolve_expr(&**right)?), *line_number))
            } else {
               bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
            }
         },
         Expr::Var(name, line_number) => {
            if let Some(unique_name) = self.variable_map.get(name) {
               return Ok(Expr::Var(Rc::clone(unique_name), *line_number));
            } else {
               bail!(error::error(*line_number, format!("Undeclared variable {}", name), error::ErrorType::SemanticError))
            }
         },
         Expr::BinaryOp(operator, left, right, line_number) => {
            Ok(Expr::BinaryOp(operator.clone(), Box::new(self.resolve_expr(left)?), Box::new(self.resolve_expr(right)?), *line_number))
         },
         Expr::Integer(i, line_number) => {
            Ok(Expr::Integer(*i, *line_number))
         },
         Expr::UnaryOp(operator, expr, line_number) => {
            Ok(Expr::UnaryOp(operator.clone(), Box::new(self.resolve_expr(expr)?), *line_number))
         },
         Expr::Conditional(condition, middle, right) => {
            Ok(Expr::Conditional(Box::new(self.resolve_expr(condition)?), Box::new(self.resolve_expr(middle)?), Box::new(self.resolve_expr(right)?)))
         }
      }
   }
}
