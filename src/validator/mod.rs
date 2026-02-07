use crate::error;
use crate::name_generator;
use crate::parser::ast_printer;
use crate::parser::ast::*;

use anyhow::{Result, bail};
use std::collections::HashMap;

type VariableMap = HashMap<String, (String, bool)>;

fn copy_variable_map(map: &HashMap<String, (String, bool)>) -> HashMap<String, (String, bool)> {
   let mut new_map = HashMap::new();
   for (key, value) in map {
      new_map.insert(key.clone(), (value.0.clone(), false));
   }
   new_map
}

pub fn validate(ast: &mut AST, print_ast: bool) -> Result<()> {
   let mut variable_map: VariableMap = HashMap::new();
   resolve_program(&mut ast.program, &mut variable_map)?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(())
}

fn resolve_program(program: &mut Program, variable_map: &mut VariableMap) -> Result<()> {
   match program {
      Program::FunctionDefinition(FunctionDefinition{ name: _, body }) => {
         resolve_function(body, variable_map)?;
         Ok(())
      }
   }
}

fn resolve_function(body: &mut Block, variable_map: &mut VariableMap) -> Result<()> {
   resolve_block(body, variable_map)
}

fn resolve_block(block: &mut Block, variable_map: &mut VariableMap) -> Result<()> {
   for block_item in &mut *block.items {
      resolve_block_item(block_item, variable_map)?;
   }
   Ok(())
}

fn resolve_block_item(item: &mut BlockItem, variable_map: &mut VariableMap) -> Result<()> {
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
         resolve_block(block, &mut new_variable_map)?;
      },
      _ => todo!()
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
