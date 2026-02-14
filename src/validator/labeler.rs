use anyhow::{Result, bail};
use crate::error;
use crate::name_generator;
use crate::parser::ast::*;

pub fn label_program(program: &mut Program)  -> Result<()> {
   for decl in &mut program.func_decls {
      label_func_declaration(decl)?;
   }
   Ok(())
}

fn label_func_declaration(decl: &mut FuncDecl) -> Result<()> {
   if let Some(body) = &mut decl.body {
      label_block(body, &None)?;
   }
   Ok(())
}

fn label_block(block: &mut Block, loop_label: &Option<String>) -> Result<()> {
   for block_item in &mut *block.items {
      label_block_item(block_item, &loop_label)?;
   }
   Ok(())
}

fn label_block_item(item: &mut BlockItem, loop_label: &Option<String>) -> Result<()> {
   match item {
      BlockItem::Stmt(stmt) => {
         label_statement(stmt, loop_label)?;
      },
      _ => ()
   }
   Ok(())
}

fn label_statement(stmt: &mut Stmt, loop_label: &Option<String>) -> Result<()> {
   match stmt {
      Stmt::Break(label, line_number) => {
         if let Some(l) = loop_label {
            *label = l.clone();
         } else {
            bail!(error::error(*line_number, "break statement outside of loop".to_string(), error::ErrorType::SemanticError))
         }
      },
      Stmt::Continue(label, line_number) => {
         if let Some(l) = loop_label {
            *label = l.clone();
         } else {
            bail!(error::error(*line_number, "continue outside of loop".to_string(), error::ErrorType::SemanticError))
         }
      },
      Stmt::While(_, body, label) => {
         let new_label = Some(name_generator::gen_label("while"));
         label_statement(body, &new_label)?;
         *label = new_label.unwrap().clone();
      },
      Stmt::DoWhile(body, _, label) => {
         let new_label = Some(name_generator::gen_label("dowhile"));
         label_statement(body, &new_label)?;
         *label = new_label.unwrap().clone();
      },
      Stmt::For(_, _, _, body, label) => {
         let new_label = Some(name_generator::gen_label("for"));
         label_statement(body, &new_label)?;
         *label = new_label.unwrap().clone();
      },
      Stmt::Compound(block) => {
         label_block(block, loop_label)?;
      },
      Stmt::If(_, then_stmt, else_stmt) => {
         label_statement(then_stmt, loop_label)?;
         label_optional_stmt(else_stmt, loop_label)?;
      },
      _ => ()
   }
   Ok(())
}

fn label_optional_stmt(stmt: &mut Option<Box<Stmt>>, loop_label: &Option<String>) -> Result<()> {
   if let Some(s) = stmt {
      label_statement(s, loop_label)?;
   }
   Ok(())
}
