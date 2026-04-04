use anyhow::{bail, Result};
use crate::error::{error, ErrorType};
use crate::parser::ast::*;

struct SwitchInfo {
}

pub fn validate(program: &mut Program) -> Result<()> {
   for decl in &mut program.decls {
      if let Decl::FuncDecl(decl) = decl {
         validate_func_decl_switch_stmts(decl)?;
      }
   }
   Ok(())
}

fn validate_func_decl_switch_stmts(func: &mut FuncDecl) -> Result<()> {
   if let Some(body) = &mut func.body {
      validate_func_switch_stmts(body)?;
   }
   Ok(())
}

fn validate_func_switch_stmts(body: &mut Block) -> Result<()> {
   validate_block_switch_stmts(body, &mut None)?;
   Ok(())
}

fn validate_block_switch_stmts(block: &mut Block, switch_info: &mut Option<SwitchInfo>) -> Result<()> {
   for block_item in &mut *block.items {
      validate_block_item_switch_stmts(block_item, switch_info)?;
   }
   Ok(())
}

fn validate_block_item_switch_stmts(block_item: &mut BlockItem, switch_info: &mut Option<SwitchInfo>) -> Result<()> {
   if let BlockItem::Stmt(stmt) = block_item {
      validate_stmt_switch_stmts(stmt, switch_info)?;
   }
   Ok(())
}

fn validate_stmt_switch_stmts(stmt: &mut Stmt, switch_info: &mut Option<SwitchInfo>) -> Result<()> {
   match stmt {
      Stmt::Return(_, labels, _) => {
         validate_labels(labels, switch_info)?
      },
      Stmt::Expression(_, labels, _) => {
         validate_labels(labels, switch_info)?
      },
      Stmt::If(_, stmt, then_stmt, labels, _) => {
         validate_labels(labels, switch_info)?;
         validate_stmt_switch_stmts(stmt, switch_info)?;
         if let Some(stmt) = then_stmt {
            validate_stmt_switch_stmts(stmt, switch_info)?;
         }
      },
      Stmt::Compound(block, labels, _) => {
         validate_labels(labels, switch_info)?;
         validate_block_switch_stmts(block, switch_info)?
      },
      Stmt::Break(_, labels, _) => {
         validate_labels(labels, switch_info)?;
      },
      Stmt::Continue(_, labels, _) => {
         validate_labels(labels, switch_info)?;
      },
      Stmt::While(_, stmt, labels, _) => {
         validate_labels(labels, switch_info)?;
         validate_stmt_switch_stmts(stmt, switch_info)?
      },
      Stmt::DoWhile(body, _, labels, _) => {
         validate_labels(labels, switch_info)?;
         validate_stmt_switch_stmts(&mut *body, switch_info)?
      },
      Stmt::For(_, _, _, stmt, labels, _) => {
         validate_labels(labels, switch_info)?;
         validate_stmt_switch_stmts(stmt, switch_info)?
      },
      Stmt::Goto(_, labels, _) => {
         validate_labels(labels, switch_info)?;
      },
      Stmt::Switch(_, stmt, labels, _) => {
         validate_labels(labels, switch_info)?;
         let new_switch_info = SwitchInfo {  };
         validate_stmt_switch_stmts(stmt, &mut Some(new_switch_info))?;
      },
      Stmt::Null(labels, _) => {
         validate_labels(labels, switch_info)?;
      }
   }
   Ok(())
}

fn validate_labels(labels: &Vec<Label>, switch_info: &Option<SwitchInfo>) -> Result<()> {
   for label in labels {
      if let None = switch_info && (label.name == "default" || label.name == "case") {
         bail!(error(label.line_number, format!("{} label outside of switch statement", label.name), ErrorType::SemanticError))
      }
   }
   Ok(())
}

