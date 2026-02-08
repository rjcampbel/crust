use crate::error;
use crate::name_generator;
use crate::parser::ast_printer;
use crate::parser::ast::*;

use anyhow::{Result, bail};
use std::collections::HashMap;

type VariableMap = HashMap<String, (String, bool)>;

pub fn validate(ast: &mut AST, print_ast: bool) -> Result<()> {
   let mut variable_map: VariableMap = HashMap::new();
   validate_program(&mut ast.program, &mut variable_map)?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(())
}

fn validate_program(program: &mut Program, variable_map: &mut VariableMap) -> Result<()> {
   match program {
      Program::FunctionDefinition(FunctionDefinition{ name: _, body }) => {
         validate_function(body, variable_map)?;
         Ok(())
      }
   }
}

fn validate_function(body: &mut Block, variable_map: &mut VariableMap) -> Result<()> {
   validate_block(body, variable_map)?;
   label_block(body, &None)?;
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

fn validate_block(block: &mut Block, variable_map: &mut VariableMap) -> Result<()> {
   for block_item in &mut *block.items {
      validate_block_item(block_item, variable_map)?;
   }
   Ok(())
}

fn validate_block_item(item: &mut BlockItem, variable_map: &mut VariableMap) -> Result<()> {
   match item {
      BlockItem::Stmt(stmt) => {
         resolve_statement(stmt, variable_map)?;
      },
      BlockItem::Decl(decl) => {
         resolve_declaration(decl, variable_map)?;
      }
   }
   Ok(())
}

fn label_statement(stmt: &mut Stmt, loop_label: &Option<String>) -> Result<()> {
   match stmt {
      Stmt::Break(label) => {
         if let Some(l) = loop_label {
            *label = l.clone();
         } else {
            bail!(error::error(0, "break statement outside of loop".to_string(), error::ErrorType::SemanticError))
         }
      },
      Stmt::Continue(label) => {
         if let Some(l) = loop_label {
            *label = l.clone();
         } else {
            bail!(error::error(0, "continue outside of loop".to_string(), error::ErrorType::SemanticError))
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

fn resolve_statement(stmt: &mut Stmt, variable_map: &mut VariableMap) -> Result<()> {
   match stmt {
      Stmt::Expression(e) => {
         resolve_expr(e, variable_map)?;
      },
      Stmt::Return(e) => {
         resolve_expr(e, variable_map)?;
      },
      Stmt::Null => (),
      Stmt::If(expr, then_stmt, else_stmt) => {
         if let Some(else_stmt) = else_stmt {
            resolve_statement(else_stmt, variable_map)?;
         }
         resolve_expr(expr, variable_map)?;
         resolve_statement(then_stmt, variable_map)?;
      },
      Stmt::Compound(block) => {
         let mut new_variable_map = copy_variable_map(variable_map);
         validate_block(block, &mut new_variable_map)?;
      },
      Stmt::Break(_) => (),
      Stmt::Continue(_) => (),
      Stmt::While(condition, body, _) => {
         resolve_expr(condition, variable_map)?;
         resolve_statement(body, variable_map)?;
      },
      Stmt::DoWhile(body, condition, _) => {
         resolve_statement(body, variable_map)?;
         resolve_expr(condition, variable_map)?;
      },
      Stmt::For(init, condition, post, body, _) => {
         let mut new_variable_map = copy_variable_map(variable_map);
         resolve_for_init(init, &mut new_variable_map)?;
         resolve_optional_expr(condition, &mut new_variable_map)?;
         resolve_optional_expr(post, &mut new_variable_map)?;
         resolve_statement(body, &mut new_variable_map)?;
      }
   }
   Ok(())
}

fn resolve_for_init(init: &mut Option<ForInit>, variable_map: &mut VariableMap) -> Result<()> {
   match init {
      Some(ForInit::Expr(e)) => {
         resolve_expr(e, variable_map)?;
      },
      Some(ForInit::Decl(d)) => {
         resolve_declaration(d, variable_map)?;
      },
      None => ()
   }
   Ok(())
}

fn resolve_optional_expr(expr: &mut Option<Expr>, variable_map: &mut VariableMap) -> Result<()> {
   if let Some(e) = expr {
      resolve_expr(e, variable_map)?;
   }
   Ok(())
}

fn resolve_declaration(decl: &mut Decl, variable_map: &mut VariableMap) -> Result<()> {
   let Decl::Decl(name, initializer, line_number) = decl;
   if variable_map.contains_key(name) && variable_map.get(name).unwrap().1 == true {
      bail!(error::error(*line_number, format!("\"{}\" already declared.", name), error::ErrorType::SemanticError))
   }
   let unique_name = name_generator::uniquify_identifier(name);
   variable_map.insert(name.clone(), (unique_name.clone(), true));

   if let Some(expr) = initializer {
      resolve_expr(expr, variable_map)?;
   }
   *name = unique_name;
   Ok(())
}

fn resolve_expr(expr: &mut Expr, variable_map: &mut VariableMap) -> Result<()> {
   match expr {
      Expr::Assignment(left, right, line_number) => {
         if let Expr::Var(_, _) = **left {
            resolve_expr(left, variable_map)?;
            resolve_expr(right, variable_map)?;
         } else {
            bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
         }
      },
      Expr::Var(name, line_number) => {
         if let Some(_entry @ (unique_name, _)) = variable_map.get(name) {
            *name = unique_name.clone();
         } else {
            bail!(error::error(*line_number, format!("Undeclared variable {}", name), error::ErrorType::SemanticError))
         }
      },
      Expr::BinaryOp(_, left, right) => {
         resolve_expr(left, variable_map)?;
         resolve_expr(right, variable_map)?;
      },
      Expr::Integer(_) => (),
      Expr::UnaryOp(_, expr) => {
         resolve_expr(expr, variable_map)?;
      },
      Expr::Conditional(condition, middle, right) => {
         resolve_expr(condition, variable_map)?;
         resolve_expr(middle, variable_map)?;
         resolve_expr(right, variable_map)?;
      }
   }
   Ok(())
}

fn copy_variable_map(map: &HashMap<String, (String, bool)>) -> HashMap<String, (String, bool)> {
   let mut new_map = HashMap::new();
   for (key, value) in map {
      new_map.insert(key.clone(), (value.0.clone(), false));
   }
   new_map
}
