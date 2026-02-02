use crate::{codegen, codegen::assembly::AssemblyAST};
use crate::{tacky, tacky::tacky::TackyAST};
use crate::emitter;
use crate::gcc;
use crate::lexer::{Lexer, token::Token};
use crate::parser::{Parser, ast::AST};
use crate::validator;

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Compiler {
   parser: Parser,
   source: String,
}

impl Compiler {
   pub fn new() -> Self {
      Self {
         parser: Parser::new(),
         source: String::new()
      }
   }

   pub fn compile(&mut self, source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<()> {
      let assembly_ast = self.codegen(&source, print_tokens, print_ast, print_tacky, print_assembly)?;
      let output = source.with_extension("s");
      emitter::emit_code(&assembly_ast, &output)?;
      gcc::assemble(&output, &source.with_extension(""))?;
      fs::remove_file(&output)?;
      Ok(())
   }

   pub fn lex(&mut self, source: &Path, print_tokens: bool) -> Result<Vec<Token>> {
      let source = self.preprocess(source)?;
      self.source = fs::read_to_string(&source)?;
      let mut lexer = Lexer::new(&self.source);
      lexer.lex(print_tokens)?;
      fs::remove_file(&source)?;
      Ok(lexer.tokens)
   }

   pub fn parse(&mut self, source: &Path, print_tokens: bool, print_ast: bool) -> Result<AST> {
      let tokens = self.lex(source, print_tokens)?;
      Ok(self.parser.parse(&tokens, print_ast)?)
   }

   pub fn validate(&mut self, source: &Path, print_tokens: bool, print_ast: bool) -> Result<AST> {
      let ast = self.parse(&source, print_tokens, print_ast)?;
      Ok(validator::validate(&ast, print_ast)?)
   }

   pub fn tacky(&mut self, source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool) -> Result<TackyAST> {
      let ast = self.validate(&source, print_tokens, print_ast)?;
      Ok(tacky::gen_tacky(&ast, print_tacky)?)
   }

   pub fn codegen(&mut self, source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<AssemblyAST> {
      let tacky = self.tacky(&source, print_tokens, print_ast, print_tacky)?;
      Ok(codegen::codegen(&tacky, print_assembly)?)
   }

   fn preprocess(&mut self, source: &Path) -> Result<PathBuf> {
      let pp_source = source.with_extension("i");
      gcc::preprocess(&source, &pp_source)?;
      Ok(pp_source)
   }
}