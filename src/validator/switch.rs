use anyhow::{bail, Result};
use crate::error::{error, ErrorType};
use crate::parser::ast::*;

pub fn validate(program: &Program) -> Result<()> {
   for decl in &program.decls {
      if let Decl::FuncDecl(decl) = decl {
         validate_func_decl_switch_stmts(decl)?;
      }
   }
   Ok(())
}

fn validate_func_decl_switch_stmts(func: &FuncDecl) -> Result<()> {
   if let Some(body) = &func.body {
      validate_func_switch_stmts(body)?;
   }
   Ok(())
}

fn validate_func_switch_stmts(body: &Block) -> Result<()> {
   validate_block_switch_stmts(body, &None)?;
   Ok(())
}

fn validate_block_switch_stmts(block: &Block, switch_info: &Option<&SwitchInfo>) -> Result<()> {
   for block_item in &*block.items {
      validate_block_item_switch_stmts(block_item, switch_info)?;
   }
   Ok(())
}

fn validate_block_item_switch_stmts(block_item: &BlockItem, switch_info: &Option<&SwitchInfo>) -> Result<()> {
   if let BlockItem::Stmt(stmt) = block_item {
      validate_stmt_switch_stmts(stmt, switch_info)?;
   }
   Ok(())
}

fn validate_stmt_switch_stmts(stmt: &Stmt, switch_info: &Option<&SwitchInfo>) -> Result<()> {
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
         validate_stmt_switch_stmts(&*body, switch_info)?
      },
      Stmt::For(_, _, _, stmt, labels, _) => {
         validate_labels(labels, switch_info)?;
         validate_stmt_switch_stmts(stmt, switch_info)?
      },
      Stmt::Goto(_, labels, _) => {
         validate_labels(labels, switch_info)?;
      },
      Stmt::Switch(_, stmt, labels, switch_info, _) => {
         validate_switch_info(switch_info)?;
         validate_labels(labels, &Some(&switch_info))?;
         validate_stmt_switch_stmts(stmt, &Some(&switch_info))?;
      },
      Stmt::Null(labels, _) => {
         validate_labels(labels, switch_info)?;
      }
   }
   Ok(())
}

fn validate_switch_info(switch_info: &SwitchInfo) -> Result<()> {
   for case in &switch_info.cases {
      let Expr::Integer(_) = case.value else {
         bail!(error(case.line_number, format!("case label must be an integer constant expression"), ErrorType::SemanticError))
      };
   }
   Ok(())
}

fn validate_labels(labels: &Vec<Label>, switch_info: &Option<&SwitchInfo>) -> Result<()> {
   for label in labels {
      if let None = switch_info && (label.name == "default" || label.name == "case") {
         bail!(error(label.line_number, format!("{} label outside of switch statement", label.name), ErrorType::SemanticError))
      }
   }
   Ok(())
}
