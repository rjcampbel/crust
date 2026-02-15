mod checker;
mod labeler;
mod resolver;

use crate::parser::ast_printer;
use crate::parser::ast::*;

use anyhow::Result;

pub fn validate(ast: &mut AST, print_ast: bool) -> Result<()> {
   resolver::resolve_program(&mut ast.program)?;
   checker::typecheck_program(&ast.program)?;
   labeler::label_program(&mut ast.program)?;

   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(())
}