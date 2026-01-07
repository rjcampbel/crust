pub mod ast;
mod ast_printer;

use anyhow::{bail, ensure, Result};
use crate::lexer::token::{Token, TokenType};
use ast::*;
use thiserror::Error;

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

fn error(line: usize, msg: String, err_type: ErrorType) -> ParseError {
   match err_type {
      ErrorType::SyntaxError => ParseError::SyntaxError { line, msg }
   }
}

pub fn parse(tokens: Vec<Token>, print_ast: bool) -> Result<AST> {
   let mut parser = Parser::new(tokens);
   let ast = parser.parse()?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(ast)
}

struct Parser {
   tokens: Vec<Token>,
   current: usize,
}

impl Parser {
   fn new(tokens: Vec<Token>) -> Self {
      Self {
         tokens,
         current: 0,
      }
   }

   fn parse(&mut self) -> Result<AST> {
      let program = self.program()?;
      Ok(AST { program })
   }

   fn program(&mut self) -> Result<Program> {
      let function = self.function()?;
      ensure!(self.at_end(), "Expected end of file after function");
      Ok(function)
   }

   fn function(&mut self) -> Result<Program> {
      self.consume(TokenType::Int)?;
      let name = self.identifier()?;
      self.consume(TokenType::OpenParen)?;
      self.consume(TokenType::Void)?;
      self.consume(TokenType::CloseParen)?;
      self.consume(TokenType::OpenBrace)?;
      let stmt = self.statement()?;
      self.consume(TokenType::CloseBrace)?;
      Ok(Program::Function { name: name, stmt: stmt })
   }

   fn identifier(&mut self) -> Result<String> {
      match self.peek().token_type {
         TokenType::Identifier => {
            self.advance();
            Ok(self.previous().lexeme.clone())
         },
         _ => {
            let t = self.peek();
            bail!(error(t.line_number, format!("Expected an identifier, found '{}'", t.lexeme), ErrorType::SyntaxError))
         }
      }
   }

   fn statement(&mut self) -> Result<Stmt> {
      self.consume(TokenType::Return)?;
      let expr = self.expression()?;
      self.consume(TokenType::Semicolon)?;
      Ok(Stmt::Return(expr))
   }

   fn expression(&mut self) -> Result<Expr> {
      let expr = self.unary()?;
      Ok(expr)
   }

   fn unary(&mut self) -> Result<Expr> {
      if self.match_token(TokenType::Dash) || self.match_token(TokenType::Tilde) {
         let operator_type = self.previous().token_type.clone();
         let expr = self.unary()?;
         return Ok(Expr::UnaryOp {
            operator: match operator_type {
               TokenType::Dash => UnaryOp::Negate,
               TokenType::Tilde => UnaryOp::Complement,
               _ => unreachable!(),
            },
            expr: Box::new(expr),
         });
      }
      self.primary()
   }

   fn primary(&mut self) -> Result<Expr> {
      match self.peek().token_type {
         TokenType::Integer(i) => {
            self.advance();
            Ok(Expr::Integer(i))
         },
         TokenType::OpenParen => {
            self.advance();
            let expr = self.expression()?;
            self.consume(TokenType::CloseParen)?;
            Ok(expr)
         },
         _ => {
            let t = self.peek();
            bail!(error(t.line_number, format!("Expected an expression, found '{}'", t.lexeme), ErrorType::SyntaxError))
         }
      }
   }

   fn consume(&mut self, token_type: TokenType) -> Result<&Token> {
      if self.check(&token_type) {
         return Ok(self.advance());
      }
      bail!(error(self.peek().line_number, format!("Expected '{}', found '{}'", token_type, self.peek().token_type), ErrorType::SyntaxError))
   }

   fn match_token(&mut self, token_type: TokenType) -> bool {
      if self.check(&token_type) {
         self.advance();
         return true;
      }
      false
   }

   fn previous(&mut self) -> &Token {
      &self.tokens[self.current - 1]
   }

   fn check(&mut self, token_type: &TokenType) -> bool {
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

   fn peek(&mut self) -> &Token {
      &self.tokens[self.current]
   }

   fn at_end(&mut self) -> bool {
      self.peek().token_type == TokenType::EOF
   }
}