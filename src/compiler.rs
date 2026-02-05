use crate::{codegen, codegen::assembly::AssemblyAST};
use crate::{lexer, lexer::token::Token};
use crate::{parser, parser::ast::AST};
use crate::{tacky, tacky::tacky::TackyAST};
use crate::emitter;
use crate::gcc;
use crate::validator;

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct Compiler {
   source_path: PathBuf,
   pp_source_path: PathBuf,
   assembly_path: PathBuf,
   source: String,
}

impl Drop for Compiler {
   fn drop(&mut self) {
      let _ = fs::remove_file(&self.assembly_path);
      let _ = fs::remove_file(&self.pp_source_path);
   }
}

impl Compiler {
   pub fn new(source_path: PathBuf) -> Result<Self> {
      let mut compiler = Self {
         source_path,
         pp_source_path: PathBuf::new(),
         assembly_path: PathBuf::new(),
         source: String::new(),
      };
      compiler.pp_source_path = compiler.source_path.with_extension("i");
      compiler.assembly_path = compiler.source_path.with_extension("s");
      Ok(compiler)
   }

   pub fn compile(&mut self, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<()> {
      let assembly_ast = self.codegen(print_tokens, print_ast, print_tacky, print_assembly)?;
      emitter::emit_code(&assembly_ast, &self.assembly_path)?;
      gcc::assemble(&self.assembly_path, &self.source_path.with_extension(""))?;
      Ok(())
   }

   pub fn preprocess(&mut self) -> Result<()> {
      gcc::preprocess(&self.source_path, &self.pp_source_path)?;
      self.source = fs::read_to_string(&self.pp_source_path)?;
      Ok(())
   }

   pub fn lex(&mut self, print_tokens: bool) -> Result<Vec<Option<Token>>> {
      self.preprocess()?;
      Ok(lexer::lex(&self.source, print_tokens)?)
   }

   pub fn parse(&mut self, print_tokens: bool, print_ast: bool) -> Result<AST> {
      let tokens = self.lex(print_tokens)?;
      Ok(parser::parse(tokens, print_ast)?)
   }

   pub fn validate(&mut self, print_tokens: bool, print_ast: bool) -> Result<AST> {
      let mut ast = self.parse(print_tokens, print_ast)?;
      validator::validate(&mut ast, print_ast)?;
      Ok(ast)
   }

   pub fn tacky(&mut self, print_tokens: bool, print_ast: bool, print_tacky: bool) -> Result<TackyAST> {
      let ast = self.validate(print_tokens, print_ast)?;
      Ok(tacky::gen_tacky(ast, print_tacky)?)
   }

   pub fn codegen(&mut self, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<AssemblyAST> {
      let tacky = self.tacky(print_tokens, print_ast, print_tacky)?;
      Ok(codegen::codegen(tacky, print_assembly)?)
   }
}