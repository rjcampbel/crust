pub mod tacky;

use crate::parser::ast::Program;
use tacky::TackyProgram;
use anyhow::Result;

pub fn gen_tacky(program: &Program, print_tacky: bool) -> Result<TackyProgram> {
    // Implementation of tacky generation goes here
    Ok(TackyProgram::Function)
}