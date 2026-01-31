use crate::parser::ast::*;
use crate::parser::ast_printer;
use anyhow::{Result, bail};
use crate::name_generator;
use std::collections::HashMap;
use crate::error;

struct Validator {
   variable_map: HashMap<String, String>
}

pub fn validate(ast: &AST, print_ast: bool) -> Result<AST> {
   let mut validator = Validator::new();
   let ast = validator.validate(ast)?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(ast)
}

impl Validator {
   fn new() -> Self {
      Self {
         variable_map: HashMap::new()
      }
   }

   fn validate(&mut self, ast: &AST) -> Result<AST> {
      Ok(AST { program: self.resolve_program(&ast.program)? })
   }

   fn resolve_program(&mut self, program: &Program) -> Result<Program> {
      match program {
         Program::FunctionDefinition(FunctionDefinition::Function(name, body)) => {
            Ok(Program::FunctionDefinition(self.resolve_function(&name, &body)?))
         }
      }
   }

   fn resolve_function(&mut self, name: &String, body: &Vec<BlockItem>) -> Result<FunctionDefinition> {
      let mut block_items = Vec::new();
      for block_item in body {
         block_items.push(self.validate_block_item(block_item)?);
      }
      Ok(FunctionDefinition::Function(name.to_string(), block_items))
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
         _ => todo!()
      }
   }

   fn resolve_declaration(&mut self, decl: &Decl) -> Result<Decl> {
      let Decl::Decl(name, initializer, line_number) = decl;
      if self.variable_map.contains_key(name) {
         bail!(error::error(*line_number, format!("\"{}\" already declared.", name), error::ErrorType::SyntaxError))
      }
      let unique_name = name_generator::uniquify_identifier(name);
      self.variable_map.insert(name.to_string(), unique_name.clone());

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
               bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SyntaxError))
            }
         },
         Expr::Var(name, line_number) => {
            if let Some(unique_name) = self.variable_map.get(name) {
               return Ok(Expr::Var(unique_name.clone(), *line_number));
            } else {
               bail!(error::error(*line_number, format!("Undeclared variable {}", name), error::ErrorType::SyntaxError))
            }
         },
         Expr::BinaryOp { operator, left, right, line_number} => {
            Ok(Expr::BinaryOp {
                  operator: operator.clone(),
                  left: Box::new(self.resolve_expr(&**left)?),
                  right: Box::new(self.resolve_expr(&**right)?),
                  line_number: *line_number })
         },
         Expr::Integer(i, line_number) => {
            Ok(Expr::Integer(*i, *line_number))
         },
         Expr::UnaryOp { operator, expr, line_number } => {
            Ok(Expr::UnaryOp { operator: operator.clone(), expr: Box::new(self.resolve_expr(&**expr)?), line_number: *line_number })
         },
         _ => todo!()
      }
   }
}
