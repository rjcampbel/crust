use anyhow::{Result, bail};
use crate::error::{self, error};
use crate::parser::ast::*;

use std::collections::HashSet;

struct SwitchInfo {
   pub cases: Option<Cases>,
   pub default: Option<Stmt>,
}

type Cases = HashSet<Expr>;

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
      Stmt::Compound(block, _, _) => {
         validate_block_switch_stmts(block, switch_info)?
      },
      Stmt::DoWhile(body, _, _, _) => {
         validate_stmt_switch_stmts(&mut *body, switch_info)?
      },
      Stmt::For(_, _, _, stmt, _, _) => {
         validate_stmt_switch_stmts(stmt, switch_info)?
     },
      Stmt::If(_, stmt, then_stmt, _, _) => {
         validate_stmt_switch_stmts(stmt, switch_info)?;
         if let Some(stmt) = then_stmt {
            validate_stmt_switch_stmts(stmt, switch_info)?;
         }
      },
      Stmt::While(_, stmt, _, _) => {
         validate_stmt_switch_stmts(stmt, switch_info)?
      },
      Stmt::Switch(_, stmt, _, _) => {
         let new_switch_info = SwitchInfo { cases: None, default: None };
         validate_stmt_switch_stmts(stmt, &mut Some(new_switch_info))?;
      },
      Stmt::Case(expr, stmt, _, line_number) => {
         let Expr::Integer(_) = expr else {
            bail!(error(*line_number, format!("case label must be a constant expression"), error::ErrorType::SemanticError));
         };
         if let Some(s) = switch_info {
            let cases = s.cases.get_or_insert_with(|| HashSet::new());
            if !cases.insert(expr.clone()) {
                bail!(error(*line_number, format!("duplicate case label"), error::ErrorType::SemanticError));
            }
            validate_stmt_switch_stmts(stmt, switch_info)?;
         } else {
            bail!(error(*line_number, format!("case statement outside of switch"), error::ErrorType::SemanticError));
         }
      },
      Stmt::Default(stmt, _, line_number) => {
         if let Some(s) = switch_info {
            if s.default.is_some() {
               bail!(error(*line_number, format!("multiple default labels in one switch"), error::ErrorType::SemanticError));
            }
            s.default = Some(*stmt.clone());
            validate_stmt_switch_stmts(stmt, switch_info)?;
         } else {
            bail!(error::error(*line_number, format!("default statement outside of switch"), error::ErrorType::SemanticError));
         }
      },
      _=> ()
   }
   Ok(())
}

