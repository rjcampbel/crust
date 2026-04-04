use anyhow::Result;
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
         let new_switch_info = SwitchInfo {  };
         validate_stmt_switch_stmts(stmt, &mut Some(new_switch_info))?;
      },
      _=> ()
   }
   Ok(())
}

