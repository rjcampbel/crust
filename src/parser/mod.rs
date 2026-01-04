pub mod ast;
mod ast_printer;

use anyhow::{bail, ensure, Result};
use crate::{Token, TokenType};
use ast::*;
use thiserror::Error;
use ast_printer::print_ast;

#[derive(Error, Debug)]
enum ParseError {
   #[error("[line {}] Syntax Error: {}", line, msg)]
   SyntaxError {
      line: usize,
      msg: String,
   },
}

enum ErrorType {
   SyntaxError,
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program> {
   let mut parser = Parser::new(tokens);
   let program = parser.parse()?;
   print_ast(&program);
   Ok(program)
}

fn error(line: usize, msg: String, err_type: ErrorType) -> ParseError {
   match err_type {
      ErrorType::SyntaxError => ParseError::SyntaxError { line, msg }
   }
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
      self.consume(TokenType::Int)?;
      let name = self.identifier()?;
      self.consume(TokenType::OpenParen)?;
      self.consume(TokenType::Void)?;
      self.consume(TokenType::CloseParen)?;
      self.consume(TokenType::OpenBrace)?;
      let stmt = self.statement()?;
      self.consume(TokenType::CloseBrace)?;
      Ok(Function { name: name, stmt: stmt })
   }

   fn identifier(&mut self) -> Result<String> {
      let t = self.peek();
      match t.token_type.clone() {
         TokenType::Identifier(i) => {
            self.advance();
            return Ok(i.clone());
         },
         _ => { bail!(error(t.line_number, format!("Expected an identifier, found '{}'", t.lexeme), ErrorType::SyntaxError)) }
      }
   }

   fn statement(&mut self) -> Result<Stmt> {
      self.consume(TokenType::Return)?;
      let expr = self.expression()?;
      self.consume(TokenType::Semicolon)?;
      Ok(Stmt::Return(expr))
   }

   fn expression(&mut self) -> Result<Expr> {
      let val = self.integer_constant()?;
      Ok(Expr::Integer(val))
   }

   fn integer_constant(&mut self) -> Result<u64> {
      let t = self.peek();
      match t.token_type.clone() {
         TokenType::Integer(i) => {
            self.advance();
            return Ok(i.clone());
         },
         _ => { bail!(error(t.line_number, format!("Expected an integer, found '{}'", t.lexeme), ErrorType::SyntaxError)) }
      }
   }

   fn consume(&mut self, token_type: TokenType) -> Result<&Token> {
      if self.check(&token_type) {
         return Ok(self.advance());
      }
      bail!(error(self.peek().line_number, format!("Expected '{}', found '{}'", token_type, self.peek().token_type), ErrorType::SyntaxError))
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