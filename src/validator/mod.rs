use crate::error;
use crate::name_generator;
use crate::parser::ast_printer;
use crate::parser::ast::*;

use anyhow::{Result, bail};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Linkage {
   None,
   Internal,
   External
}

struct IdentifierInfo {
   unique_name: String,
   from_current_scope: bool,
   linkage: Linkage
}

type IdentifierMap = HashMap<String, IdentifierInfo>;

pub fn validate(ast: &mut AST, print_ast: bool) -> Result<()> {
   let mut identifier_map: IdentifierMap = HashMap::new();
   validate_program(&mut ast.program, &mut identifier_map)?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(())
}

fn validate_program(program: &mut Program, identifier_map: &mut IdentifierMap) -> Result<()> {
   for decl in &mut program.func_decls {
      validate_func_declaration(decl, identifier_map)?;
   }
   Ok(())
}

fn validate_func_declaration(decl: &mut FuncDecl, identifier_map: &mut IdentifierMap) -> Result<()> {
   if let Some(prev_decl) =  identifier_map.get(&decl.name) {
      if prev_decl.from_current_scope && prev_decl.linkage != Linkage::External {
         bail!(error::error(decl.line_number, format!("\"{}\" already declared.", decl.name), error::ErrorType::SemanticError))
      }
   } else {
      identifier_map.insert(decl.name.clone(), IdentifierInfo{ unique_name: decl.name.clone(), from_current_scope: true, linkage: Linkage::External });
   }

   let mut inner_map = copy_identifier_map(identifier_map);
   if !decl.params.is_empty() {
      for param in &mut decl.params {
         resolve_local_var(param, decl.line_number, &mut inner_map)?;
      }
   }
   if let Some(body) = &mut decl.body {
      validate_block(body, &mut inner_map)?;
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

fn validate_block(block: &mut Block, identifier_map: &mut IdentifierMap) -> Result<()> {
   for block_item in &mut *block.items {
      validate_block_item(block_item, identifier_map)?;
   }
   Ok(())
}

fn validate_block_item(item: &mut BlockItem, identifier_map: &mut IdentifierMap) -> Result<()> {
   match item {
      BlockItem::Stmt(stmt) => {
         resolve_statement(stmt, identifier_map)?;
      },
      BlockItem::Decl(decl) => {
         match decl {
            Decl::VarDecl(decl) => {
               resolve_var_declaration(decl, identifier_map)?;
            },
            Decl::FuncDecl(decl) => {
               validate_func_declaration(decl, identifier_map)?;
            }
         }
      }
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

fn resolve_statement(stmt: &mut Stmt, identifier_map: &mut IdentifierMap) -> Result<()> {
   match stmt {
      Stmt::Expression(e) => {
         resolve_expr(e, identifier_map)?;
      },
      Stmt::Return(e) => {
         resolve_expr(e, identifier_map)?;
      },
      Stmt::Null => (),
      Stmt::If(expr, then_stmt, else_stmt) => {
         if let Some(else_stmt) = else_stmt {
            resolve_statement(else_stmt, identifier_map)?;
         }
         resolve_expr(expr, identifier_map)?;
         resolve_statement(then_stmt, identifier_map)?;
      },
      Stmt::Compound(block) => {
         let mut new_variable_map = copy_identifier_map(identifier_map);
         validate_block(block, &mut new_variable_map)?;
      },
      Stmt::Break(_, _) => (),
      Stmt::Continue(_, _) => (),
      Stmt::While(condition, body, _) => {
         resolve_expr(condition, identifier_map)?;
         resolve_statement(body, identifier_map)?;
      },
      Stmt::DoWhile(body, condition, _) => {
         resolve_statement(body, identifier_map)?;
         resolve_expr(condition, identifier_map)?;
      },
      Stmt::For(init, condition, post, body, _) => {
         let mut new_variable_map = copy_identifier_map(identifier_map);
         resolve_for_init(init, &mut new_variable_map)?;
         resolve_optional_expr(condition, &mut new_variable_map)?;
         resolve_optional_expr(post, &mut new_variable_map)?;
         resolve_statement(body, &mut new_variable_map)?;
      }
   }
   Ok(())
}

fn resolve_for_init(init: &mut Option<ForInit>, identifier_map: &mut IdentifierMap) -> Result<()> {
   match init {
      Some(ForInit::Expr(e)) => {
         resolve_expr(e, identifier_map)?;
      },
      Some(ForInit::Decl(d)) => {
         resolve_var_declaration(d, identifier_map)?;
      },
      None => ()
   }
   Ok(())
}

fn resolve_optional_expr(expr: &mut Option<Expr>, identifier_map: &mut IdentifierMap) -> Result<()> {
   if let Some(e) = expr {
      resolve_expr(e, identifier_map)?;
   }
   Ok(())
}

fn resolve_local_var(name: &mut String, line_number: usize, identifier_map: &mut IdentifierMap) -> Result<()> {
   if identifier_map.contains_key(name) && identifier_map.get(name).unwrap().from_current_scope == true {
      bail!(error::error(line_number, format!("\"{}\" already declared.", name), error::ErrorType::SemanticError))
   }
   let unique_name = name_generator::uniquify_identifier(name);
   identifier_map.insert(name.clone(), IdentifierInfo{ unique_name: unique_name.clone(), from_current_scope: true, linkage: Linkage::None });
   *name = unique_name;
   Ok(())
}

fn resolve_var_declaration(decl: &mut VarDecl, identifier_map: &mut IdentifierMap) -> Result<()> {
   resolve_local_var(&mut decl.name, decl.line_number, identifier_map)?;

   if let Some(expr) = &mut decl.init {
      resolve_expr(expr, identifier_map)?;
   }
   Ok(())
}

fn resolve_expr(expr: &mut Expr, identifier_map: &mut IdentifierMap) -> Result<()> {
   match expr {
      Expr::Assignment(left, right, line_number) => {
         if let Expr::Var(_, _) = **left {
            resolve_expr(left, identifier_map)?;
            resolve_expr(right, identifier_map)?;
         } else {
            bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
         }
      },
      Expr::Var(name, line_number) => {
         if let Some(_entry @ IdentifierInfo {unique_name, ..}) = identifier_map.get(name) {
            *name = unique_name.clone();
         } else {
            bail!(error::error(*line_number, format!("Undeclared variable {}", name), error::ErrorType::SemanticError))
         }
      },
      Expr::BinaryOp(_, left, right) => {
         resolve_expr(left, identifier_map)?;
         resolve_expr(right, identifier_map)?;
      },
      Expr::Integer(_) => (),
      Expr::UnaryOp(_, expr) => {
         resolve_expr(expr, identifier_map)?;
      },
      Expr::Conditional(condition, middle, right) => {
         resolve_expr(condition, identifier_map)?;
         resolve_expr(middle, identifier_map)?;
         resolve_expr(right, identifier_map)?;
      },
      Expr::FunctionCall(name, args , line_number) => {
         if let Some(id_info) = identifier_map.get(name) {
            *name = id_info.unique_name.clone();
            for arg in args {
               resolve_expr(arg, identifier_map)?;
            }
         } else {
            bail!(error::error(*line_number, format!("Undeclared function {}", name), error::ErrorType::SemanticError))
         }
      }
   }
   Ok(())
}

fn copy_identifier_map(map: &HashMap<String, IdentifierInfo>) -> HashMap<String, IdentifierInfo> {
   let mut new_map = HashMap::new();
   for (key, value) in map {
      new_map.insert(key.clone(), IdentifierInfo {
         unique_name: value.unique_name.clone(),
         from_current_scope: false,
         linkage: value.linkage
      });
   }
   new_map
}
