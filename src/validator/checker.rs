use crate::{error, parser::ast::*};

use anyhow::{Result, bail};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum DeclType {
   Int,
   Func(usize)
}

#[derive(Copy, Clone)]
enum InitialValue {
   Tentative,
   Initialized(i64),
   NoInitializer
}

enum Attrs {
   FuncAttr {
      defined: bool,
      global: bool
   },
   StaticAttr {
      initial_value: InitialValue,
      global: bool
   },
   LocalAttr
}
struct TypeInfo {
   pub decl_type: DeclType,
   pub attrs: Attrs
}

type TypeMap = HashMap<String, TypeInfo>;

pub fn typecheck_program(program: &Program) -> Result<()> {
   let mut type_map = TypeMap::new();
   for decl in &program.decls {
      match decl {
         Decl::VarDecl(decl) => {
            typecheck_global_var_decl(decl, &mut type_map)?;
         },
         Decl::FuncDecl(decl) => {
            typecheck_func_declaration(decl, &mut type_map, false)?;
         }
      }
   }
   Ok(())
}

fn typecheck_global_var_decl(decl: &VarDecl, type_map: &mut TypeMap) -> Result<()> {
   let mut initial_value =
      match decl.init {
         Some(ref init) => {
            if let Expr::Integer(i) = init {
               InitialValue::Initialized(*i)
            } else {
               bail!(error::error(decl.line_number, format!("Global variable initializer must be a constant"), error::ErrorType::SemanticError))
            }
         },
         None => {
            if decl.storage_class == Some(StorageClass::Extern) {
               InitialValue::NoInitializer
            } else {
               InitialValue::Tentative
            }
         }
      };

   let mut global = decl.storage_class != Some(StorageClass::Static);

   if let Some(existing_decl) = type_map.get(&decl.name) {
      if existing_decl.decl_type != DeclType::Int {
         bail!(error::error(decl.line_number, format!("\"{}\" redeclared as a variable", decl.name), error::ErrorType::SemanticError))
      }

      let (existing_initial_value, existing_global) = match existing_decl.attrs {
         Attrs::StaticAttr { initial_value, global } => (initial_value, global),
         _ => unreachable!()
      };

      if decl.storage_class == Some(StorageClass::Extern) {
         global = existing_global;
      } else if existing_global != global {
         bail!(error::error(decl.line_number, format!("Conflicting storage class specifiers for \"{}\".", decl.name), error::ErrorType::SemanticError))
      }

      if matches!(existing_initial_value, InitialValue::Initialized(_)) {
         if matches!(initial_value, InitialValue::Initialized(_))  {
            bail!(error::error(decl.line_number, format!("Conflicting file scope variable definitions"), error::ErrorType::SemanticError))
         } else {
            initial_value = existing_initial_value;
         }
      } else if matches!(existing_initial_value, InitialValue::Tentative) && !matches!(initial_value, InitialValue::Initialized(_)){
         initial_value = InitialValue::Tentative;
      }
   }

   let attrs = Attrs::StaticAttr { initial_value, global };
   type_map.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs });
   Ok(())
}

fn typecheck_func_declaration(decl: &FuncDecl, type_map: &mut TypeMap, block_scope: bool) -> Result<()> {
   let decl_type = DeclType::Func(decl.params.len());
   let (has_body, body) =
      if let Some(body) = &decl.body {
         (true, Some(body))
      } else {
         (false, None)
      };
   let mut already_defined = false;
   let mut global = decl.storage_class != Some(StorageClass::Static);

   if !global && block_scope {
      bail!(error::error(decl.line_number, format!("Static function declaration not allowed in block scope"), error::ErrorType::SemanticError))
   }

   if let Some(existing_decl) = type_map.get(&decl.name) {
      match existing_decl.decl_type {
         DeclType::Func(p) if p == decl.params.len() => {
            match existing_decl.attrs {
               Attrs::FuncAttr { defined, .. } => {
                  already_defined = defined;
               },
               _ => unreachable!()
            }
            if already_defined && has_body {
               bail!(error::error(decl.line_number, "Function is defined more than once".to_string(), error::ErrorType::SemanticError))
            }
         },
         _ => {
            bail!(error::error(decl.line_number, format!("Incompatible function declarations"), error::ErrorType::SemanticError))
         }
      }
      match existing_decl.attrs {
         Attrs::FuncAttr { global: old_global, .. } => {
            if old_global && decl.storage_class == Some(StorageClass::Static) {
               bail!(error::error(decl.line_number, format!("Conflicting storage class specifiers for function"), error::ErrorType::SemanticError))
            }
            global = old_global;
         },
         _ => unreachable!()
      }
   }

   let defined = already_defined || has_body;
   let attrs = Attrs::FuncAttr { defined, global };
   type_map.insert(decl.name.clone(), TypeInfo{ decl_type, attrs });

   if let Some(body) = body {
      for param in &decl.params {
         type_map.insert(param.clone(), TypeInfo{ decl_type: DeclType::Int, attrs: Attrs::LocalAttr });
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
               typecheck_local_var_decl(decl, type_map)?;
            },
            Decl::FuncDecl(decl) => {
               typecheck_func_declaration(decl, type_map, true)?;
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
         if d.storage_class == Some(StorageClass::Static) {
            bail!(error::error(d.line_number, format!("Static variable declaration not allowed in for loop initializer"), error::ErrorType::SemanticError))
         }
         typecheck_local_var_decl(d, type_map)?;
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

fn typecheck_local_var_decl(decl: &VarDecl, type_map: &mut TypeMap) -> Result<()> {
   if decl.storage_class == Some(StorageClass::Extern) {
      if let Some(_) = &decl.init {
         bail!(error::error(decl.line_number, format!("Initializer on local extern variable declaration"), error::ErrorType::SemanticError))
      }
      if let Some(existing_decl) = type_map.get(&decl.name) {
         if !matches!(existing_decl.decl_type, DeclType::Int) {
            bail!(error::error(decl.line_number, format!("Function redeclared as a variable"), error::ErrorType::SemanticError))
         }
      } else {
         let attrs = Attrs::StaticAttr { initial_value: InitialValue::NoInitializer, global: true };
         type_map.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs });
      }
   } else if decl.storage_class == Some(StorageClass::Static) {
      let initial_value =
         match decl.init {
            Some(ref init) => {
               if let Expr::Integer(i) = init {
                  InitialValue::Initialized(*i)
               } else {
                  bail!(error::error(decl.line_number, format!("Global variable initializer must be a constant"), error::ErrorType::SemanticError))
               }
            },
            None => {
               InitialValue::Initialized(0)
            }
         };
      let attrs = Attrs::StaticAttr { initial_value, global: false };
      type_map.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs });
   } else {
      type_map.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs: Attrs::LocalAttr });
      if let Some(init) = &decl.init {
         typecheck_expr(init, type_map)?;
      }
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