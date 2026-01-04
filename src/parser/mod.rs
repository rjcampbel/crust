mod ast;

use anyhow::{bail, ensure, Result};
use crate::{Token, TokenType};
use ast::*;

pub fn parse(tokens: &Vec<Token>) -> Result<()> {
   let mut parser = Parser::new(tokens);
   parser.parse()?;
   Ok(())
}

struct Parser<'a> {
   tokens: &'a Vec<Token>,
   current: usize,
}

impl<'a> Parser<'a> {
   fn new(tokens: &'a Vec<Token>) -> Self {
      Self {
         tokens,
         current: 0,
      }
   }

   fn parse(&mut self) -> Result<Program> {
      let program = self.program()?;
      Ok(program)
   }

   fn program(&mut self) -> Result<Program> {
      let function = self.function()?;
      ensure!(self.at_end(), "Expected end of file after function");
      Ok(Program::Function(function))
   }

   fn function(&mut self) -> Result<Function> {
      self.consume(TokenType::Int, "Expected int")?;
      let name = self.identifier()?;
      self.consume(TokenType::OpenParen, "Expected (")?;
      self.consume(TokenType::Void, "Expected void")?;
      self.consume(TokenType::CloseParen, "Expected )")?;
      self.consume(TokenType::OpenBrace, "Expected {")?;
      let stmt = self.statement()?;
      self.consume(TokenType::CloseBrace, "Expected }")?;
      Ok(Function { name: name, stmt: stmt })
   }

   fn identifier(&mut self) -> Result<String> {
      match self.peek().token_type.clone() {
         TokenType::Identifier(i) => {
            self.advance();
            return Ok(i.clone());
         },
         _ => { bail!("Expected an identifier") }
      }
   }

   fn statement(&mut self) -> Result<Stmt> {
      self.consume(TokenType::Return, "Expected return")?;
      let expr = self.expression()?;
      self.consume(TokenType::Semicolon, "Expected ;")?;
      Ok(Stmt::Return(expr))
   }

   fn expression(&mut self) -> Result<Expr> {
      let val = self.integer_constant()?;
      Ok(Expr::Integer(val))
   }

   fn integer_constant(&mut self) -> Result<u64> {
      match self.peek().token_type.clone() {
         TokenType::Integer(i) => {
            self.advance();
            return Ok(i.clone());
         },
         _ => { bail!("Expected an integer") }
      }
   }

   fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token> {
      if self.check(&token_type) {
         return Ok(self.advance());
      }
      bail!(msg.to_string())
   }

   fn check(&self, token_type: &TokenType) -> bool {
      if self.at_end() {
         return false;
      }
      &self.peek().token_type == token_type
   }

   fn advance(&mut self) -> &Token {
      let t = &self.tokens[self.current];
      self.current += 1;
      return t;
   }

   fn peek(&self) -> &Token {
      &self.tokens[self.current]
   }

   fn at_end(&self) -> bool {
      self.peek().token_type == TokenType::EOF
   }
}