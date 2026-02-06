use crate::error;
use crate::name_generator;
use crate::parser::ast_printer;
use crate::parser::ast::*;

use anyhow::{Result, bail};
use std::collections::HashMap;

struct Validator {
   variable_map: HashMap<String, String>
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
         Program::FunctionDefinition(FunctionDefinition{ name: _, body }) => {
            self.resolve_function(body)?;
            Ok(())
         }
      }
   }

   fn resolve_function(&mut self, body: &mut Block) -> Result<()> {
      for block_item in &mut *body.items {
         self.validate_block_item(block_item)?;
      }
      Ok(())
   }

   fn validate_block_item(&mut self, item: &mut BlockItem) -> Result<()> {
      match item {
         BlockItem::Stmt(stmt) => {
            self.resolve_statement(stmt)?;
         },
         BlockItem::Decl(decl) => {
            self.resolve_declaration(decl)?;
         }
      }
      Ok(())
   }

   fn resolve_statement(&mut self, stmt: &mut Stmt) -> Result<()> {
      match stmt {
         Stmt::Expression(e) => {
            self.resolve_expr(e)?;
         },
         Stmt::Return(e) => {
            self.resolve_expr(e)?;
         },
         Stmt::Null => (),
         Stmt::If(expr, then_stmt, else_stmt) => {
            if let Some(else_stmt) = else_stmt {
               self.resolve_statement(else_stmt)?;
            }
            self.resolve_expr(expr)?;
            self.resolve_statement(then_stmt)?;
         },
         Stmt::Compound(block) => {
            for block_item in &mut *block.items {
               self.validate_block_item(block_item)?;
            }
         }
      }
      Ok(())
   }

   fn resolve_declaration(&mut self, decl: &mut Decl) -> Result<()> {
      let Decl::Decl(name, initializer, line_number) = decl;
      if self.variable_map.contains_key(name) {
         bail!(error::error(*line_number, format!("\"{}\" already declared.", name), error::ErrorType::SemanticError))
      }
      let unique_name = name_generator::uniquify_identifier(name);
      self.variable_map.insert(name.clone(), unique_name.clone());

      if let Some(expr) = initializer {
         self.resolve_expr(expr)?;
      }
      *name = unique_name;
      Ok(())
   }

   fn resolve_expr(&mut self, expr: &mut Expr) -> Result<()> {
      match expr {
         Expr::Assignment(left, right, line_number) => {
            if let Expr::Var(_, _) = **left {
               self.resolve_expr(left)?;
               self.resolve_expr(right)?;
            } else {
               bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
            }
         },
         Expr::Var(name, line_number) => {
            if let Some(unique_name) = self.variable_map.get(name) {
               *name = unique_name.clone();
            } else {
               bail!(error::error(*line_number, format!("Undeclared variable {}", name), error::ErrorType::SemanticError))
            }
         },
         Expr::BinaryOp(_, left, right) => {
            self.resolve_expr(left)?;
            self.resolve_expr(right)?;
         },
         Expr::Integer(_) => (),
         Expr::UnaryOp(_, expr) => {
            self.resolve_expr(expr)?;
         },
         Expr::Conditional(condition, middle, right) => {
            self.resolve_expr(condition)?;
            self.resolve_expr(middle)?;
            self.resolve_expr(right)?;
         }
      }
      Ok(())
   }
}
