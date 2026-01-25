use crate::parser::ast::*;
use crate::parser::ast_printer;
use anyhow::{Result, bail};
use crate::name_generator;
use std::collections::HashMap;

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
         Stmt::Null => Ok(Stmt::Null)
      }
   }

   fn resolve_declaration(&mut self, decl: &Decl) -> Result<Decl> {
      bail!("unimpl")
   }

   fn resolve_expr(&mut self, expr: &Expr) -> Result<Expr> {
      bail!("unimpl")
   }
}
