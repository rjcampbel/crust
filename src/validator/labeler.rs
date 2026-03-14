use anyhow::{Result, bail};
use crate::error;
use crate::name_generator;
use crate::parser::ast::*;

pub fn label_program(program: &mut Program)  -> Result<()> {
   for decl in &mut program.decls {
      if let Decl::FuncDecl(decl) = decl {
         label_func_decl(decl)?;
      }
   }
   Ok(())
}

fn label_func_decl(decl: &mut FuncDecl) -> Result<()> {
   if let Some(body) = &mut decl.body {
      label_block(body, &None)?;
   }
   Ok(())
}

fn label_block(block: &mut Block, loop_label: &Option<Label>) -> Result<()> {
   for block_item in &mut *block.items {
      label_block_item(block_item, &loop_label)?;
   }
   Ok(())
}

fn label_block_item(item: &mut BlockItem, loop_label: &Option<Label>) -> Result<()> {
   match item {
      BlockItem::Stmt(stmt) => {
         label_statement(stmt, loop_label)?;
      },
      _ => ()
   }
   Ok(())
}

fn label_statement(stmt: &mut Stmt, loop_label: &Option<Label>) -> Result<()> {
   match stmt {
      Stmt::Break(label, _, line_number) => {
         if let Some(l) = loop_label {
            *label = l.clone();
         } else {
            bail!(error::error(*line_number, "break statement outside of loop".to_string(), error::ErrorType::SemanticError))
         }
      },
      Stmt::Continue(label, _, line_number) => {
         if let Some(l) = loop_label {
            *label = l.clone();
         } else {
            bail!(error::error(*line_number, "continue outside of loop".to_string(), error::ErrorType::SemanticError))
         }
      },
      Stmt::While(_, body, labels, line_number) => {
         let new_label = Label::new(name_generator::gen_label("while"), *line_number);
         label_statement(body, &Some(new_label.clone()))?;
         labels.push(new_label);
      },
      Stmt::DoWhile(body, _, labels, line_number) => {
         let new_label = Label::new(name_generator::gen_label("dowhile"), *line_number);
         label_statement(body, &Some(new_label.clone()))?;
         labels.push(new_label);
      },
      Stmt::For(_, _, _, body, labels, line_number) => {
         let new_label = Label::new(name_generator::gen_label("for"), *line_number);
         label_statement(body, &Some(new_label.clone()))?;
         labels.push(new_label);
      },
      Stmt::Compound(block, _, _) => {
         label_block(block, loop_label)?;
      },
      Stmt::If(_, then_stmt, else_stmt, _, _) => {
         label_statement(then_stmt, loop_label)?;
         label_optional_stmt(else_stmt, loop_label)?;
      },
      _ => ()
   }
   Ok(())
}

fn label_optional_stmt(stmt: &mut Option<Box<Stmt>>, loop_label: &Option<Label>) -> Result<()> {
   if let Some(s) = stmt {
      label_statement(s, loop_label)?;
   }
   Ok(())
}
