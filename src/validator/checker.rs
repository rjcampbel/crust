use crate::{error, parser::ast::*};
use super::symbol_table::*;
use anyhow::{Result, bail};

pub fn typecheck_ast(ast: &mut AST) -> Result<()> {
   typecheck_program(&ast.program, &mut ast.symbol_table)
}

fn typecheck_program(program: &Program, symbol_table: &mut SymbolTable) -> Result<()> {
   for decl in &program.decls {
      match decl {
         Decl::VarDecl(decl) => {
            typecheck_global_var_decl(decl, symbol_table)?;
         },
         Decl::FuncDecl(decl) => {
            typecheck_func_decl(decl, symbol_table, false)?;
         }
      }
   }
   Ok(())
}

fn typecheck_global_var_decl(decl: &VarDecl, symbol_table: &mut SymbolTable) -> Result<()> {
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

   if let Some(existing_decl) = symbol_table.get(&decl.name) {
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
   symbol_table.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs });
   Ok(())
}

fn typecheck_func_decl(decl: &FuncDecl, symbol_table: &mut SymbolTable, block_scope: bool) -> Result<()> {
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

   if let Some(existing_decl) = symbol_table.get(&decl.name) {
      match existing_decl.decl_type {
         DeclType::Func(p) if p == decl.params.len() => {
            match existing_decl.attrs {
               Attrs::FuncAttr { defined, global: old_global } => {
                  already_defined = defined;
                  if already_defined && has_body {
                     bail!(error::error(decl.line_number, "Function is defined more than once".to_string(), error::ErrorType::SemanticError))
                  }
                  if old_global && decl.storage_class == Some(StorageClass::Static) {
                     bail!(error::error(decl.line_number, format!("Conflicting storage class specifiers for function"), error::ErrorType::SemanticError))
                  }
                  global = old_global;
               },
               _ => unreachable!()
            }
         },
         _ => {
            bail!(error::error(decl.line_number, format!("Incompatible function declarations"), error::ErrorType::SemanticError))
         }
      }
   }

   let defined = already_defined || has_body;
   let attrs = Attrs::FuncAttr { defined, global };
   symbol_table.insert(decl.name.clone(), TypeInfo{ decl_type, attrs });

   if let Some(body) = body {
      for param in &decl.params {
         symbol_table.insert(param.clone(), TypeInfo{ decl_type: DeclType::Int, attrs: Attrs::LocalAttr });
      }
      typecheck_block(&body, symbol_table)?;
   }
   Ok(())
}

fn typecheck_block(block: &Block, symbol_table: &mut SymbolTable) -> Result<()> {
   for block_item in &block.items {
      typecheck_block_item(block_item, symbol_table)?;
   }
   Ok(())
}

fn typecheck_block_item(block_item: &BlockItem, symbol_table: &mut SymbolTable) -> Result<()> {
   match block_item {
      BlockItem::Stmt(stmt) => {
         typecheck_statement(stmt, symbol_table)?;
      },
      BlockItem::Decl(decl) => {
         match decl {
            Decl::VarDecl(decl) => {
               typecheck_local_var_decl(decl, symbol_table)?;
            },
            Decl::FuncDecl(decl) => {
               typecheck_func_decl(decl, symbol_table, true)?;
            }
         }
      }
   }
   Ok(())
}

fn typecheck_statement(stmt: &Stmt, symbol_table: &mut SymbolTable) -> Result<()> {
   match stmt {
      Stmt::Expression(e, _, _) => {
         typecheck_expr(e, symbol_table)?;
      },
      Stmt::Return(e, _, _) => {
         typecheck_expr(e, symbol_table)?;
      },
      Stmt::Null(_, _) => (),
      Stmt::If(expr, then_stmt, else_stmt, _, _) => {
         if let Some(else_stmt) = else_stmt {
            typecheck_statement(else_stmt, symbol_table)?;
         }
         typecheck_expr(expr, symbol_table)?;
         typecheck_statement(then_stmt, symbol_table)?;
      },
      Stmt::Compound(block, _, _) => {
         typecheck_block(block, symbol_table)?;
      },
      Stmt::Break(_, _, _) => (),
      Stmt::Continue(_, _, _) => (),
      Stmt::While(condition, body, _, _) => {
         typecheck_expr(condition, symbol_table)?;
         typecheck_statement(body, symbol_table)?;
      },
      Stmt::DoWhile(body, condition, _, _) => {
         typecheck_statement(body, symbol_table)?;
         typecheck_expr(condition, symbol_table)?;
      },
      Stmt::For(init, condition, post, body, _, _) => {
         typecheck_for_init(init, symbol_table)?;
         typecheck_optional_expr(condition, symbol_table)?;
         typecheck_optional_expr(post, symbol_table)?;
         typecheck_statement(body, symbol_table)?;
      }
      Stmt::Goto(..) => (),
      Stmt::Switch(expr, stmt, _, _) => {
         typecheck_expr(expr, symbol_table)?;
         typecheck_statement(stmt, symbol_table)?;
      },
      Stmt::Case(expr, stmt, _, _) => {
         typecheck_expr(expr, symbol_table)?;
         typecheck_statement(stmt, symbol_table)?;
      },
      Stmt::Default(stmt, _, _) => {
         typecheck_statement(stmt, symbol_table)?;
      }
   }
   Ok(())
}

fn typecheck_for_init(init: &Option<ForInit>, symbol_table: &mut SymbolTable) -> Result<()> {
   match init {
      Some(ForInit::Expr(e)) => {
         typecheck_expr(e, symbol_table)?;
      },
      Some(ForInit::Decl(d)) => {
         if d.storage_class == Some(StorageClass::Static) {
            bail!(error::error(d.line_number, format!("Static variable declaration not allowed in for loop initializer"), error::ErrorType::SemanticError))
         }
         typecheck_local_var_decl(d, symbol_table)?;
      },
      None => ()
   }
   Ok(())
}

fn typecheck_optional_expr(expr: &Option<Expr>, symbol_table: &mut SymbolTable) -> Result<()> {
   if let Some(e) = expr {
      typecheck_expr(e, symbol_table)?;
   }
   Ok(())
}

fn typecheck_local_var_decl(decl: &VarDecl, symbol_table: &mut SymbolTable) -> Result<()> {
   if decl.storage_class == Some(StorageClass::Extern) {
      if let Some(_) = &decl.init {
         bail!(error::error(decl.line_number, format!("Initializer on local extern variable declaration"), error::ErrorType::SemanticError))
      }
      if let Some(existing_decl) = symbol_table.get(&decl.name) {
         if !matches!(existing_decl.decl_type, DeclType::Int) {
            bail!(error::error(decl.line_number, format!("Function redeclared as a variable"), error::ErrorType::SemanticError))
         }
      } else {
         let attrs = Attrs::StaticAttr { initial_value: InitialValue::NoInitializer, global: true };
         symbol_table.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs });
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
      symbol_table.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs });
   } else {
      symbol_table.insert(decl.name.clone(), TypeInfo { decl_type: DeclType::Int, attrs: Attrs::LocalAttr });
      if let Some(init) = &decl.init {
         typecheck_expr(init, symbol_table)?;
      }
   }
   Ok(())
}

fn typecheck_expr(expr: &Expr, symbol_table: &mut SymbolTable) -> Result<()> {
   match expr {
      Expr::Assignment(left, right, line_number) => {
         if let Expr::Var(_, _) = **left {
            typecheck_expr(left, symbol_table)?;
            typecheck_expr(right, symbol_table)?;
         } else {
            bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
         }
      },
      Expr::Var(name, line_number) => {
         if let Some(t) = symbol_table.get(name) {
            if t.decl_type != DeclType::Int {
               bail!(error::error(*line_number, "Function name used as variable".to_string(), error::ErrorType::SemanticError))
            }
         }
      },
      Expr::BinaryOp(_, left, right) => {
         typecheck_expr(left, symbol_table)?;
         typecheck_expr(right, symbol_table)?;
      },
      Expr::Integer(_) => (),
      Expr::UnaryOp(UnaryOp::PreIncrement | UnaryOp::PreDecrement | UnaryOp::PostIncrement | UnaryOp::PostDecrement, expr, line_number) => {
         if let Expr::Var(_, _) = **expr {
            typecheck_expr(expr, symbol_table)?;
         } else {
            bail!(error::error(*line_number, format!("Invalid lvalue"), error::ErrorType::SemanticError))
         }
      }
      Expr::UnaryOp(_, expr, _) => {
         typecheck_expr(expr, symbol_table)?;
      },
      Expr::Conditional(condition, middle, right) => {
         typecheck_expr(condition, symbol_table)?;
         typecheck_expr(middle, symbol_table)?;
         typecheck_expr(right, symbol_table)?;
      },
      Expr::FunctionCall(name, args , line_number) => {
         if let Some(t) = symbol_table.get(name) {
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
               typecheck_expr(arg, symbol_table)?;
            }
         }
      }
   }
   Ok(())
}