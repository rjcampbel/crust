pub mod ast;
mod ast_printer;

use anyhow::{bail, ensure, Result};
use crate::lexer::token::{Token, TokenType};
use ast::*;
use thiserror::Error;
use num::traits::FromPrimitive;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
   None,
   LogicalOr,
   LogicalAnd,
   BitwiseOr,
   BitwiseXor,
   BitwiseAnd,
   Equality,
   Comparison,
   Shift,
   Term,
   Factor,
   Max,
}

impl Precedence {
   fn increment(&self) -> Precedence {
      match FromPrimitive::from_u8(*self as u8 + 1) {
         Some(p) => p,
         _ => Precedence::Max,
      }
   }
}

impl FromPrimitive for Precedence {
   fn from_i64(n: i64) -> Option<Self> {
      match n {
         0 => Some(Precedence::None),
         1 => Some(Precedence::LogicalOr),
         2 => Some(Precedence::LogicalAnd),
         3 => Some(Precedence::BitwiseOr),
         4 => Some(Precedence::BitwiseXor),
         5 => Some(Precedence::BitwiseAnd),
         6 => Some(Precedence::Equality),
         7 => Some(Precedence::Comparison),
         8 => Some(Precedence::Shift),
         9 => Some(Precedence::Term),
         10 => Some(Precedence::Factor),
         _ => Some(Precedence::Max),
      }
   }

   fn from_u64(n: u64) -> Option<Self> {
      match n {
         0 => Some(Precedence::None),
         1 => Some(Precedence::LogicalOr),
         2 => Some(Precedence::LogicalAnd),
         3 => Some(Precedence::BitwiseOr),
         4 => Some(Precedence::BitwiseXor),
         5 => Some(Precedence::BitwiseAnd),
         6 => Some(Precedence::Equality),
         7 => Some(Precedence::Comparison),
         8 => Some(Precedence::Shift),
         9 => Some(Precedence::Term),
         10 => Some(Precedence::Factor),
         _ => Some(Precedence::Max),
      }
   }
}

impl TokenType {
   fn precedence(&self) -> Precedence {
      match self {
         TokenType::Star => Precedence::Factor,
         TokenType::Slash => Precedence::Factor,
         TokenType::Percent => Precedence::Factor,
         TokenType::Plus => Precedence::Term,
         TokenType::Dash => Precedence::Term,
         TokenType::DoubleLess => Precedence::Shift,
         TokenType::DoubleGreater => Precedence::Shift,
         TokenType::Pipe => Precedence::BitwiseOr,
         TokenType::Caret => Precedence::BitwiseXor,
         TokenType::Ampersand => Precedence::BitwiseAnd,
         _ => Precedence::None,
      }
   }
}

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
      let expr = self.expression(Precedence::None)?;
      self.consume(TokenType::Semicolon)?;
      Ok(Stmt::Return(expr))
   }

   fn expression(&mut self, min_prec: Precedence) -> Result<Expr> {
      let mut left: Expr = self.factor()?;

      while self.peek().token_type.precedence() >= min_prec && self.binary_op() {
         let operator_type = self.advance().token_type.clone();
         let next_prec = operator_type.precedence().increment();
         let right = self.expression(next_prec)?;
         left = Expr::BinaryOp {
            operator: match operator_type {
               TokenType::Plus => BinaryOp::Add,
               TokenType::Dash => BinaryOp::Subtract,
               TokenType::Star => BinaryOp::Multiply,
               TokenType::Slash => BinaryOp::Divide,
               TokenType::Percent => BinaryOp::Modulus,
               TokenType::Ampersand => BinaryOp::BitwiseAnd,
               TokenType::Pipe => BinaryOp::BitwiseOr,
               TokenType::Caret => BinaryOp::BitwiseXor,
               TokenType::DoubleLess => BinaryOp::LeftShift,
               TokenType::DoubleGreater => BinaryOp::RightShift,
               TokenType::DoubleAmpersand => BinaryOp::LogicalAnd,
               TokenType::DoublePipe => BinaryOp::LogicalOr,
               TokenType::DoubleEqual => BinaryOp::Equal,
               TokenType::BangEqual => BinaryOp::NotEqual,
               TokenType::Less => BinaryOp::LessThan,
               TokenType::LessOrEqual => BinaryOp::LessOrEqual,
               TokenType::Greater => BinaryOp::GreaterThan,
               TokenType::GreaterOrEqual => BinaryOp::GreaterOrEqual,
               _ => bail!("Unsupported operator")
            },
            left: Box::new(left.clone()),
            right: Box::new(right)
         };
      }
      Ok(left)
   }

   fn unary(&mut self) -> Result<Expr> {
      let operator_type = self.previous().token_type.clone();
      let expr = self.factor()?;
      Ok(Expr::UnaryOp {
         operator: match operator_type {
            TokenType::Dash => UnaryOp::Negate,
            TokenType::Tilde => UnaryOp::Complement,
            TokenType::Bang => UnaryOp::Not,
            _ => unreachable!(),
         },
         expr: Box::new(expr),
      })
   }

   fn factor(&mut self) -> Result<Expr> {
      match self.peek().token_type {
         TokenType::Integer(i) => {
            self.advance();
            Ok(Expr::Integer(i))
         },
         TokenType::Tilde | TokenType::Dash | TokenType::Bang => {
            self.advance();
            self.unary()
         },
         TokenType::OpenParen => {
            self.advance();
            let precedence = self.peek().token_type.precedence();
            let expr = self.expression(precedence)?;
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

   // fn match_token(&mut self, token_type: TokenType) -> bool {
   //    if self.check(&token_type) {
   //       self.advance();
   //       return true;
   //    }
   //    false
   // }

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

   fn binary_op(&mut self) -> bool {
      match self.peek().token_type {
            TokenType::Plus |
            TokenType::Dash |
            TokenType::Star |
            TokenType::Slash |
            TokenType::Percent |
            TokenType::Ampersand |
            TokenType::Pipe |
            TokenType::Caret |
            TokenType::DoubleLess |
            TokenType::DoubleGreater |
            TokenType::DoubleAmpersand |
            TokenType::DoublePipe |
            TokenType::DoubleEqual |
            TokenType::BangEqual |
            TokenType::Less |
            TokenType::LessOrEqual |
            TokenType::Greater |
            TokenType::GreaterOrEqual
            => true,
         _ => false,
      }
   }
}