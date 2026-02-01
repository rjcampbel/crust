use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::validator;
use crate::tacky;
use crate::codegen;
use crate::emitter;
use crate::gcc;
use anyhow::Result;
use std::path::Path;

pub struct Compiler {
   lexer: Lexer,
   parser: Parser,
}

impl Compiler {
   pub fn new() -> Self {
      Self {
         lexer: Lexer::new2(),
         parser: Parser::new2(),
      }
   }

   pub fn compile(&mut self, source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<()> {
      self.lexer.lex2(source, print_tokens)?;
      let ast = self.parser.parse2(&self.lexer.tokens, print_ast)?;
      let ast = validator::validate(&ast, print_ast)?;
      let tacky = tacky::gen_tacky(&ast, print_tacky)?;
      let assembly_ast = codegen::codegen(&tacky, print_assembly)?;
      let output = source.with_extension("s");
      emitter::emit_code(&assembly_ast, &output)?;
      gcc::assemble(&output, &source.with_extension(""))?;
      Ok(())
   }

   pub fn lex(&mut self, source: &Path, print_tokens: bool) -> Result<()> {
      self.lexer.lex2(source, print_tokens)
   }

   pub fn parse(&mut self, source: &Path, print_tokens: bool, print_ast: bool) -> Result<()> {
      self.lexer.lex2(source, print_tokens)?;
      let _ = self.parser.parse2(&self.lexer.tokens, print_ast)?;
      Ok(())
   }

   pub fn validate(&mut self, source: &Path, print_tokens: bool, print_ast: bool) -> Result<()> {
      self.lexer.lex2(source, print_tokens)?;
      let ast = self.parser.parse2(&self.lexer.tokens, print_ast)?;
      let _ = validator::validate(&ast, print_ast)?;
      Ok(())
   }

   pub fn tacky(&mut self, source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool) -> Result<()> {
      self.lexer.lex2(source, print_tokens)?;
      let ast = self.parser.parse2(&self.lexer.tokens, print_ast)?;
      let ast = validator::validate(&ast, print_ast)?;
      let _ = tacky::gen_tacky(&ast, print_tacky)?;
      Ok(())
   }

   pub fn codegen(&mut self, source: &Path, print_tokens: bool, print_ast: bool, print_tacky: bool, print_assembly: bool) -> Result<()> {
      self.lexer.lex2(source, print_tokens)?;
      let ast = self.parser.parse2(&self.lexer.tokens, print_ast)?;
      let ast = validator::validate(&ast, print_ast)?;
      let tacky = tacky::gen_tacky(&ast, print_tacky)?;
      let _ = codegen::codegen(&tacky, print_assembly)?;
      Ok(())
   }
}