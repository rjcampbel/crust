use crate::parser::ast::*;
use crate::parser::ast_printer;
use anyhow::{Result, bail};

pub fn validate(ast: &AST, print_ast: bool) -> Result<AST> {
   let program = resolve_program(&ast.program)?;

   if print_ast {
      ast_printer::print_ast(&ast);
   }

   Ok(AST { program: program })
}

pub fn resolve_program(program: &Program) -> Result<Program> {
   match program {
      Program::FunctionDefinition(FunctionDefinition::Function(name, body)) => {
         Ok(Program::FunctionDefinition(resolve_function(&name, &body)?))
      }
   }
}

pub fn resolve_function(name: &String, body: &Vec<BlockItem>) -> Result<FunctionDefinition> {
   let mut block_items = Vec::new();
   for block_item in body {
      block_items.push(validate_block_item(block_item)?);
   }
   Ok(FunctionDefinition::Function(name.to_string(), block_items))
}

pub fn validate_block_item(item: &BlockItem) -> Result<BlockItem> {
   match item {
      BlockItem::Stmt(stmt) => {
         Ok(BlockItem::Stmt(resolve_statement(stmt)?))
      },
      BlockItem::Decl(decl) => {
         Ok(BlockItem::Decl(resolve_declaration(decl)?))
      }
   }
}

pub fn resolve_statement(stmt: &Stmt) -> Result<Stmt> {
   match stmt {
      Stmt::Expression(e) => {
         Ok(Stmt::Expression(resolve_expr(&e)?))
      },
      Stmt::Return(e) => {
         Ok(Stmt::Return(resolve_expr(&e)?))
      },
      Stmt::Null => Ok(Stmt::Null)
   }
}

pub fn resolve_declaration(decl: &Decl) -> Result<Decl> {
   bail!("unimpl")
}

pub fn resolve_expr(expr: &Expr) -> Result<Expr> {
   bail!("unimpl")
}