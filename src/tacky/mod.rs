pub mod tacky;

use crate::parser::ast::AST;
use tacky::*;
use anyhow::Result;

pub fn gen_tacky(ast: &AST, print_tacky: bool) -> Result<TackyAST> {
    // Implementation of tacky generation goes here
    Ok(TackyAST { program: TackyProgram::Function })
}
