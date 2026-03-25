use anyhow::{Result, bail};
use crate::error::{self, error};
use crate::name_generator;
use crate::parser::ast::*;

use std::collections::HashMap;

type Labels = HashMap<String, String>;

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
   validate_func_goto_stmts(body, &func_labels)?;
   Ok(())
}

fn validate_func_goto_stmts(body: &mut Block, labels: &Labels) -> Result<()> {
   for block_item in &mut body.items {
      validate_block_item_goto_stmts(block_item, labels)?;
   }
   Ok(())
}

fn validate_block_item_goto_stmts(block_item: &mut BlockItem, labels: &Labels) -> Result<()> {
   if let BlockItem::Stmt(stmt) = block_item {
      validate_stmt_goto_stmts(stmt, labels)?;
   }
   Ok(())
}

fn validate_block_goto_stmts(block: &mut Block, labels: &Labels) -> Result<()> {
   for block_item in &mut block.items {
      validate_block_item_goto_stmts(block_item, labels)?;
   }
   Ok(())
}

fn validate_stmt_goto_stmts(stmt: &mut Stmt, labels: &Labels) -> Result<()> {
   match stmt {
      Stmt::Break(label, _, line_number) => {
         validate_jump_label(&mut label.name, labels, *line_number)?
      },
      Stmt::Compound(block, _, _) => {
         validate_block_goto_stmts(block, labels)?
      },
      Stmt::Continue(label, _, line_number) => {
         validate_jump_label(&mut label.name, labels, *line_number)?
      },
      Stmt::DoWhile(body, _, _, _) => {
         validate_stmt_goto_stmts(&mut *body, labels)?
      },
      Stmt::Expression(_, _, _) => (),
      Stmt::For(_, _, _, stmt, _, _) => {
         validate_stmt_goto_stmts(stmt, labels)?
     },
      Stmt::Goto(label, _, line_number) => {
         validate_jump_label(label, labels, *line_number)?
      },
      Stmt::If(_, stmt, then_stmt, _, _) => {
         validate_stmt_goto_stmts(stmt, labels)?;
         if let Some(stmt) = then_stmt {
            validate_stmt_goto_stmts(stmt, labels)?;
         }
      },
      Stmt::Null(_, _) => (),
      Stmt::Return(_, _, _) => (),
      Stmt::While(_, stmt, _, _) => {
         validate_stmt_goto_stmts(stmt, labels)?
      },
      _ => todo!()
   }
   Ok(())
}

fn validate_jump_label(label: &mut String, labels: &Labels, line_number: usize) -> Result<()> {
   if let Some(unique_label) = labels.get(label) {
      *label = unique_label.clone();
   } else {
      bail!(error(line_number, format!("Label not found: {}", label), error::ErrorType::SemanticError))
   };
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
      },
      _ => todo!()
   }
   Ok(())
}

fn validate_labels(stmt_labels: &mut Vec<Label>, func_labels: &mut Labels) -> Result<()> {
   for stmt_label in stmt_labels {
      validate_label(stmt_label, func_labels)?;
   }
   Ok(())
}

fn validate_label(stmt_label: &mut Label, func_labels: &mut Labels) -> Result<()> {
   if let Some(label) = func_labels.get(&stmt_label.name) {
      bail!(error(stmt_label.line_number, format!("Duplicate label: {}", label), error::ErrorType::SemanticError))
   } else {
      let unique_name = name_generator::uniquify_identifier(&stmt_label.name);
      func_labels.insert(stmt_label.name.clone(), unique_name.clone());
      stmt_label.name = unique_name.clone();
   }
   Ok(())
}

