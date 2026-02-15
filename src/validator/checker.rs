use crate::{error, parser::ast::*};

use anyhow::{Result, bail};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum DeclType {
   Int,
   Func(usize)
}

struct TypeInfo {
   pub decl_type: DeclType,
   pub defined: bool
}

type TypeMap = HashMap<String, TypeInfo>;

pub fn typecheck_program(program: &Program) -> Result<()> {
   let mut type_map = TypeMap::new();
   for decl in &program.func_decls {
      typecheck_func_declaration(&decl, &mut type_map)?;
   }
   Ok(())
}

fn typecheck_func_declaration(decl: &FuncDecl, type_map: &mut TypeMap) -> Result<()> {
   let decl_type = DeclType::Func(decl.params.len());
   let mut already_defined = false;

   let (has_body, body) = if let Some(body) = &decl.body {
      (true, Some(body))
   } else {
      (false, None)
   };

   if let Some(existing_decl_type) = type_map.get(&decl.name) {
      if existing_decl_type.decl_type != decl_type {
         bail!(error::error(decl.line_number, "Incompatible function declarations".to_string(), error::ErrorType::SemanticError))
      }
      already_defined = existing_decl_type.defined;
      if already_defined && has_body {
         bail!(error::error(decl.line_number, "Function is defined more than once".to_string(), error::ErrorType::SemanticError))
      }
   }

   type_map.insert(decl.name.clone(), TypeInfo{ decl_type, defined: already_defined || has_body });

   if let Some(body) = body {
      for param in &decl.params {
         type_map.insert(param.clone(), TypeInfo{ decl_type: DeclType::Int, defined: false });
      }
      typecheck_block(&body, type_map)?;
   }
   Ok(())
}

fn typecheck_block(block: &Block, type_map: &mut TypeMap) -> Result<()> {
   for block_item in &block.items {
      typecheck_block_item(block_item, type_map)?;
   }
   Ok(())
}

fn typecheck_block_item(block_item: &BlockItem, type_map: &mut TypeMap) -> Result<()> {
   match block_item {
      BlockItem::Stmt(stmt) => {
         typecheck_statement(stmt, type_map)?;
      },
      BlockItem::Decl(decl) => {
         match decl {
            Decl::VarDecl(decl) => {
               typecheck_var_declaration(decl, type_map)?;
            },
            Decl::FuncDecl(decl) => {
               typecheck_func_declaration(decl, type_map)?;
            }
         }
      }
   }
   Ok(())
}

fn typecheck_statement(stmt: &Stmt, type_map: &mut TypeMap) -> Result<()> {
   match stmt {
      Stmt::Expression(e) => {
         typecheck_expr(e, type_map)?;
      },
      Stmt::Return(e) => {
         typecheck_expr(e, type_map)?;
      },
      Stmt::Null => (),
      Stmt::If(expr, then_stmt, else_stmt) => {
         if let Some(else_stmt) = else_stmt {
            typecheck_statement(else_stmt, type_map)?;
         }
         typecheck_expr(expr, type_map)?;
         typecheck_statement(then_stmt, type_map)?;
      },
      Stmt::Compound(block) => {
         typecheck_block(block, type_map)?;
      },
      Stmt::Break(_, _) => (),
      Stmt::Continue(_, _) => (),
      Stmt::While(condition, body, _) => {
         typecheck_expr(condition, type_map)?;
         typecheck_statement(body, type_map)?;
      },
      Stmt::DoWhile(body, condition, _) => {
         typecheck_statement(body, type_map)?;
         typecheck_expr(condition, type_map)?;
      },
      Stmt::For(init, condition, post, body, _) => {
         typecheck_for_init(init, type_map)?;
         typecheck_optional_expr(condition, type_map)?;
         typecheck_optional_expr(post, type_map)?;
         typecheck_statement(body, type_map)?;
      }
   }
   Ok(())
}

fn typecheck_for_init(init: &Option<ForInit>, type_map: &mut TypeMap) -> Result<()> {
   match init {
      Some(ForInit::Expr(e)) => {
         typecheck_expr(e, type_map)?;
      },
      Some(ForInit::Decl(d)) => {
         typecheck_var_declaration(d, type_map)?;
      },
      None => ()
   }
   Ok(())
}

fn typecheck_optional_expr(expr: &Option<Expr>, type_map: &mut TypeMap) -> Result<()> {
   if let Some(e) = expr {
      typecheck_expr(e, type_map)?;
   }
   Ok(())
}

fn typecheck_var_declaration(decl: &VarDecl, type_map: &mut TypeMap) -> Result<()> {
   type_map.insert(decl.name.clone(), TypeInfo{ decl_type: DeclType::Int, defined: false });
   if let Some(init) = &decl.init {
      typecheck_expr(init, type_map)?;
   }
   Ok(())
}

fn typecheck_expr(expr: &Expr, type_map: &mut TypeMap) -> Result<()> {
   match expr {
      Expr::Assignment(left, right, line_number) => {
         if let Expr::Var(_, _) = **left {
            typecheck_expr(left, type_map)?;
            typecheck_expr(right, type_map)?;
         } else {
            bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
         }
      },
      Expr::Var(name, line_number) => {
         if let Some(t) = type_map.get(name) {
            if t.decl_type != DeclType::Int {
               bail!(error::error(*line_number, "Function name used as variable".to_string(), error::ErrorType::SemanticError))
            }
         }
      },
      Expr::BinaryOp(_, left, right) => {
         typecheck_expr(left, type_map)?;
         typecheck_expr(right, type_map)?;
      },
      Expr::Integer(_) => (),
      Expr::UnaryOp(_, expr) => {
         typecheck_expr(expr, type_map)?;
      },
      Expr::Conditional(condition, middle, right) => {
         typecheck_expr(condition, type_map)?;
         typecheck_expr(middle, type_map)?;
         typecheck_expr(right, type_map)?;
      },
      Expr::FunctionCall(name, args , line_number) => {
         if let Some(t) = type_map.get(name) {
            match t.decl_type {
               DeclType::Int => {
                  bail!(error::error(*line_number, "Variable used as function name".to_string(), error::ErrorType::SemanticError))
               },
               DeclType::Func(num_args) => {
                  if num_args != args.len() {
                     bail!(error::error(*line_number, "Function called with the wrong number of arguments".to_string(), error::ErrorType::SemanticError))
                  }
               }
            }
            for arg in args {
               typecheck_expr(arg, type_map)?;
            }
         }
      }
   }
   Ok(())
}