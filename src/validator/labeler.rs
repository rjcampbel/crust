use anyhow::{Result, bail};
use crate::error;
use crate::name_generator;
use crate::parser::ast::*;

enum Context {
   Loop,
   Switch
}

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
      let mut context_stack = Vec::new();
      label_block(body, &None, &None, false, &mut context_stack)?;
   }
   Ok(())
}

fn label_block(block: &mut Block, loop_label: &Option<String>, switch_end_label: &Option<String>, in_loop: bool, context_stack: &mut Vec<Context>) -> Result<()> {
   for block_item in &mut *block.items {
      label_block_item(block_item, &loop_label, switch_end_label, in_loop, context_stack)?;
   }
   Ok(())
}

fn label_block_item(item: &mut BlockItem, loop_label: &Option<String>, switch_end_label: &Option<String>, in_loop: bool, context_stack: &mut Vec<Context>) -> Result<()> {
   match item {
      BlockItem::Stmt(stmt) => {
         label_statement(stmt, loop_label, switch_end_label, in_loop, context_stack)?;
      },
      _ => ()
   }
   Ok(())
}

fn label_statement(stmt: &mut Stmt, loop_label: &Option<String>, switch_end_label: &Option<String>, in_loop: bool, context_stack: &mut Vec<Context>) -> Result<()> {
   match stmt {
      Stmt::Break(label, _, line_number) => {
         if let Some(context) = context_stack.last() {
            match context {
               Context::Loop => {
                  *label = loop_label.as_ref().unwrap().clone();
               },
               Context::Switch => {
                  *label = switch_end_label.as_ref().unwrap().clone();
               }
            }
         } else {
            bail!(error::error(*line_number, format!("break statement outside of loop or switch statement"), error::ErrorType::SemanticError))
         }
      },
      Stmt::Continue(label, _, line_number) => {
         if !in_loop {
            bail!(error::error(*line_number, format!("continue statement outside of loop"), error::ErrorType::SemanticError))
         }
         *label = loop_label.as_ref().unwrap().clone();
      },
      Stmt::While(_, body, labels, line_number) => {
         let label_name = name_generator::gen_label("while");
         let new_label = Label::new(label_name.clone(), *line_number);
         context_stack.push(Context::Loop);
         label_statement(body, &Some(label_name), switch_end_label, true, context_stack)?;
         context_stack.pop();
         labels.push(new_label);
      },
      Stmt::DoWhile(body, _, labels, line_number) => {
         let label_name = name_generator::gen_label("dowhile");
         let new_label = Label::new(label_name.clone(), *line_number);
         context_stack.push(Context::Loop);
         label_statement(body, &Some(label_name), switch_end_label, true, context_stack)?;
         context_stack.pop();
         labels.push(new_label);
      },
      Stmt::For(_, _, _, body, labels, line_number) => {
         let label_name = name_generator::gen_label("for");
         let new_label = Label::new(label_name.clone(), *line_number);
         context_stack.push(Context::Loop);
         label_statement(body, &Some(label_name), switch_end_label, true, context_stack)?;
         context_stack.pop();
         labels.push(new_label);
      },
      Stmt::Compound(block, _, _) => {
         label_block(block, loop_label, switch_end_label, in_loop, context_stack)?;
      },
      Stmt::If(_, then_stmt, else_stmt, _, _) => {
         label_statement(then_stmt, loop_label, switch_end_label, in_loop, context_stack)?;
         label_optional_stmt(else_stmt, loop_label, switch_end_label, in_loop, context_stack)?;
      },
      Stmt::Switch(_, stmt, _, switch_info, _) => {
         context_stack.push(Context::Switch);
         label_statement(stmt, loop_label, &Some(switch_info.end_label.clone()), in_loop, context_stack)?;
         context_stack.pop();
      },
      _ => ()
   }
   Ok(())
}

fn label_optional_stmt(stmt: &mut Option<Box<Stmt>>, loop_label: &Option<String>, switch_end_label: &Option<String>, in_loop: bool, context_stack: &mut Vec<Context>) -> Result<()> {
   if let Some(s) = stmt {
      label_statement(s, loop_label, &switch_end_label, in_loop, context_stack)?;
   }
   Ok(())
}
