use anyhow::{Result, bail};
use crate::error::{self, error};
use crate::parser::ast::*;

use std::collections::HashSet;

type Labels = HashSet<Label>;

pub fn validate(program: &mut Program) -> Result<()> {
   for decl in &mut program.decls {
      if let Decl::FuncDecl(decl) = decl {
         validate_func_decl_labels(decl)?;
      }
   }
   Ok(())
}

fn validate_func_decl_labels(func: &mut FuncDecl) -> Result<()> {
   if let Some(body) = &mut func.body {
      validate_func_labels(body)?;
   }
   Ok(())
}

fn validate_func_labels(body: &mut Block) -> Result<()> {
   let mut func_labels = Labels::new();
   for block_item in &mut *body.items {
      validate_block_item_labels(block_item, &mut func_labels)?;
   }
   Ok(())
}

fn validate_block_item_labels(item: &mut BlockItem, labels: &mut Labels) -> Result<()> {
   match item {
      BlockItem::Stmt(stmt) => {
         validate_stmt_labels(stmt, labels)?;
      },
      BlockItem::Decl(_) => ()
   }
   Ok(())
}

fn validate_block_labels(block: &mut Block, labels: &mut Labels) -> Result<()> {
   for block_item in &mut *block.items {
      validate_block_item_labels(block_item, labels)?;
   }
   Ok(())
}

fn validate_stmt_labels(stmt: &mut Stmt, labels: &mut Labels) -> Result<()> {
   match stmt {
      Stmt::Break(_, stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?
      },
      Stmt::Compound(block, stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?;
         validate_block_labels(block, labels)?
      },
      Stmt::Continue(_, stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?
      },
      Stmt::DoWhile(body, _, stmt_labels, _) => {
         validate_stmt_labels(body, labels)?;
         validate_labels(stmt_labels, labels)?
      },
      Stmt::Expression(_, stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?
      },
      Stmt::For(_, _, _, stmt, stmt_labels, _) => {
         validate_stmt_labels(stmt, labels)?;
         validate_labels(stmt_labels, labels)?
      },
      Stmt::Goto(_, stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?
      },
      Stmt::If(_, stmt, then_stmt, stmt_labels, _) => {
         validate_stmt_labels(stmt, labels)?;
         if let Some(stmt) = then_stmt {
            validate_stmt_labels(stmt, labels)?;
         }
         validate_labels(stmt_labels, labels)?
      },
      Stmt::Null(stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?
      },
      Stmt::Return(_, stmt_labels, _) => {
         validate_labels(stmt_labels, labels)?
      },
      Stmt::While(_, stmt, stmt_labels, _) => {
         validate_stmt_labels(stmt, labels)?;
         validate_labels(stmt_labels, labels)?
      }
   }
   Ok(())
}

fn validate_labels(stmt_labels: &Vec<Label>, func_labels: &mut Labels) -> Result<()> {
   for stmt_label in stmt_labels {
      if !func_labels.insert(stmt_label.clone()) {
         bail!(error(stmt_label.line_number, format!("Duplicate label: {}", stmt_label.name), error::ErrorType::SemanticError))
      }
   }
   Ok(())
}

