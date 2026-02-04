pub mod ast;
pub mod ast_printer;

use crate::error;
use crate::lexer::token::{Token, TokenType};

use anyhow::{bail, ensure, Result};
use ast::*;
use num::traits::FromPrimitive;
use std::rc::Rc;

#[derive(PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
#[repr(u8)]
enum Precedence {
   None,
   Assignment,
   Ternary,
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
         TokenType::DoubleAmpersand => Precedence::LogicalAnd,
         TokenType::DoublePipe => Precedence::LogicalOr,
         TokenType::Less => Precedence::Comparison,
         TokenType::LessOrEqual => Precedence::Comparison,
         TokenType::Greater => Precedence::Comparison,
         TokenType::GreaterOrEqual => Precedence::Comparison,
         TokenType::DoubleEqual => Precedence::Equality,
         TokenType::BangEqual => Precedence::Equality,
         TokenType::Equal => Precedence::Assignment,
         TokenType::PlusEqual => Precedence::Assignment,
         TokenType::MinusEqual => Precedence::Assignment,
         TokenType::StarEqual => Precedence::Assignment,
         TokenType::SlashEqual => Precedence::Assignment,
         TokenType::PercentEqual => Precedence::Assignment,
         TokenType::AndEqual => Precedence::Assignment,
         TokenType::OrEqual => Precedence::Assignment,
         TokenType::XorEqual => Precedence::Assignment,
         TokenType::LeftShiftEqual => Precedence::Assignment,
         TokenType::RightShiftEqual => Precedence::Assignment,
         TokenType::Question => Precedence::Ternary,
         _ => Precedence::None,
      }
   }

   pub const BINARY_OPS: &[Self] = &[
      TokenType::Plus,
      TokenType::Dash,
      TokenType::Star,
      TokenType::Slash,
      TokenType::Percent,
      TokenType::Ampersand,
      TokenType::Pipe,
      TokenType::Caret,
      TokenType::DoubleLess,
      TokenType::DoubleGreater,
      TokenType::DoubleAmpersand,
      TokenType::DoublePipe,
      TokenType::DoubleEqual,
      TokenType::BangEqual,
      TokenType::Less,
      TokenType::LessOrEqual,
      TokenType::Greater,
      TokenType::GreaterOrEqual,
   ];

   fn to_binary_op(&self) -> BinaryOp {
      match self {
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
         _ => unreachable!()
      }
   }

   pub const UNARY_OPS: &[Self] = &[
      TokenType::Dash,
      TokenType::Tilde,
      TokenType::Bang];

   fn to_unary_op(&self) -> UnaryOp {
      match self {
         TokenType::Dash => UnaryOp::Negate,
         TokenType::Tilde => UnaryOp::Complement,
         TokenType::Bang => UnaryOp::Not,
         _ => unreachable!()
      }
   }

   pub const ASSIGNMENT_OPS: &[Self] = &[
      TokenType::Equal,
      TokenType::PlusEqual,
      TokenType::MinusEqual,
      TokenType::StarEqual,
      TokenType::SlashEqual,
      TokenType::PercentEqual,
      TokenType::AndEqual,
      TokenType::OrEqual,
      TokenType::XorEqual,
      TokenType::LeftShiftEqual,
      TokenType::RightShiftEqual,
   ];
}

fn compound_to_arithmetic(t: &TokenType) -> BinaryOp {
   match t {
      TokenType::PlusEqual => BinaryOp::Add,
      TokenType::MinusEqual => BinaryOp::Subtract,
      TokenType::StarEqual => BinaryOp::Multiply,
      TokenType::SlashEqual => BinaryOp::Divide,
      TokenType::PercentEqual => BinaryOp::Modulus,
      TokenType::AndEqual => BinaryOp::BitwiseAnd,
      TokenType::OrEqual => BinaryOp::BitwiseOr,
      TokenType::XorEqual => BinaryOp::BitwiseXor,
      TokenType::LeftShiftEqual => BinaryOp::LeftShift,
      TokenType::RightShiftEqual => BinaryOp::RightShift,
      _ => unreachable!()
   }
}

struct Parser {
   tokens: Vec<Option<Token>>,
   current: usize,
}

pub fn parse(tokens: Vec<Option<Token>>, print_ast: bool) -> Result<AST> {
   let mut parser = Parser::new(tokens);
   Ok(parser.parse(print_ast)?)
}

impl Parser {
   pub fn new(tokens: Vec<Option<Token>>) -> Self {
      Self {
         tokens,
         current: 0,
      }
   }

   pub fn parse(&mut self, print_ast: bool) -> Result<AST> {
      let program = self.program()?;
      let ast = AST { program };
      if print_ast {
         ast_printer::print_ast(&ast);
      }

      Ok(ast)
   }

   fn program(&mut self) -> Result<Program> {
      let function = self.function()?;
      ensure!(self.at_end(), error::error(self.peek().as_ref().unwrap().line_number,
                              format!("Expected end of file, found {}", self.peek().as_ref().unwrap().lexeme),
                              error::ErrorType::SyntaxError));
      Ok(function)
   }

   fn function(&mut self) -> Result<Program> {
      self.consume(TokenType::Int)?;
      let name = self.identifier()?;
      self.consume(TokenType::OpenParen)?;
      self.consume(TokenType::Void)?;
      self.consume(TokenType::CloseParen)?;
      self.consume(TokenType::OpenBrace)?;
      let body = self.body()?;
      self.consume(TokenType::CloseBrace)?;
      Ok(Program::FunctionDefinition(FunctionDefinition::Function(name, body)))
   }

   fn body(&mut self) -> Result<Vec<BlockItem>> {
      let mut body = Vec::new();
      while !self.at_end() && self.peek().as_ref().unwrap().token_type != TokenType::CloseBrace {
         let block_item = self.block_item()?;
         body.push(block_item);
      }
      Ok(body)
   }

   fn block_item(&mut self) -> Result<BlockItem> {
      match self.peek().as_ref().unwrap().token_type {
         TokenType::Int => {
            self.declaration()
         },
         _ => Ok(BlockItem::Stmt(self.statement()?))
      }
   }

   fn declaration(&mut self) -> Result<BlockItem> {
      self.consume(TokenType::Int)?;
      let name = self.identifier()?;
      let expr = if self.peek().as_ref().unwrap().token_type == TokenType::Equal {
         self.advance();
         Some(self.expression(Precedence::None)?)
      } else {
         None
      };
      let line = self.peek().as_ref().unwrap().line_number;
      self.consume(TokenType::Semicolon)?;
      Ok(BlockItem::Decl(Decl::Decl(name, expr, line)))
   }

   fn identifier(&mut self) -> Result<Rc<String>> {
      match self.peek().as_ref().unwrap().token_type {
         TokenType::Identifier => {
            self.advance();
            Ok(self.previous().take().unwrap().lexeme)
         },
         _ => {
            let t = self.peek();
            bail!(error::error(t.as_ref().unwrap().line_number,
                  format!("Expected an identifier, found '{}'", t.as_ref().unwrap().lexeme),
                  error::ErrorType::SyntaxError))
         }
      }
   }

   fn statement(&mut self) -> Result<Stmt> {
      match self.peek().as_ref().unwrap().token_type {
         TokenType::Return => {
            self.advance();
            let expr = self.expression(Precedence::None)?;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Return(expr))
         },
         TokenType::Semicolon => {
            self.advance();
            return Ok(Stmt::Null);
         },
         TokenType::If => {
            self.advance();
            self.consume(TokenType::OpenParen)?;
            let expr = self.expression(Precedence::None)?;
            self.consume(TokenType::CloseParen)?;
            let then_stmt = self.statement()?;
            let else_stmt = if self.peek().as_ref().unwrap().token_type == TokenType::Else {
               self.advance();
               Some(Box::new(self.statement()?))
            } else {
               None
            };
            return Ok(Stmt::If(expr, Box::new(then_stmt), else_stmt));
         },
         _ => {
            let expr = self.expression(Precedence::None)?;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Expression(expr));
         }
      }
   }

   fn expression(&mut self, min_prec: Precedence) -> Result<Expr> {
      let mut left: Expr = self.factor()?;

      while self.peek().as_ref().unwrap().token_type.precedence() >= min_prec {
         if self.match_binary_op() {
            let line_number = self.previous().as_ref().unwrap().line_number;
            let next_prec = self.previous().as_ref().unwrap().token_type.precedence().increment();
            let binary_op = self.previous().as_ref().unwrap().token_type.to_binary_op();
            let right = self.expression(next_prec)?;
            left = Expr::BinaryOp(binary_op, Box::new(left), Box::new(right), line_number);
         } else if self.match_assignment_op() {
            match self.previous().as_ref().unwrap().token_type {
               TokenType::Equal => {
                  let prec = self.previous().as_ref().unwrap().token_type.precedence();
                  let line_number = self.previous().as_ref().unwrap().line_number;
                  let right = self.expression(prec)?;
                  left = Expr::Assignment(Box::new(left), Box::new(right), line_number);
               },
               ref t @ _ => {
                  let op = compound_to_arithmetic(t);
                  left = self.compound_assignment(op, left)?;
               },
            }
         } else if self.match_token(TokenType::Question) {
            let prec = self.previous().as_ref().unwrap().token_type.precedence();
            let middle = self.expression(Precedence::None)?;
            self.consume(TokenType::Colon)?;
            let right = self.expression(prec)?;
            left = Expr::Conditional(Box::new(left), Box::new(middle), Box::new(right));
         } else {
            break;
         }
      }
      Ok(left)
   }

   fn compound_assignment(&mut self, op: BinaryOp, left: Expr) -> Result<Expr> {
      let line_number = self.previous().as_ref().unwrap().line_number;
      let prec = self.previous().as_ref().unwrap().token_type.precedence();
      let right = Expr::BinaryOp(op, Box::new(left.clone()), Box::new(self.expression(prec)?), line_number);
      Ok(Expr::Assignment(Box::new(left), Box::new(right), line_number))
   }

   fn unary(&mut self) -> Result<Expr> {
      let line_number = self.previous().as_ref().unwrap().line_number;
      let unary_op = self.previous().as_ref().unwrap().token_type.to_unary_op();
      let expr = self.factor()?;
      Ok(Expr::UnaryOp(unary_op, Box::new(expr), line_number))
   }

   fn factor(&mut self) -> Result<Expr> {
      if self.match_unary_op() {
         return self.unary()
      } else {
         match self.peek().as_ref().unwrap().token_type {
            TokenType::Integer(i) => {
               let line_number = self.peek().as_ref().unwrap().line_number;
               self.advance();
               Ok(Expr::Integer(i, line_number))
            },
            TokenType::OpenParen => {
               self.advance();
               let precedence = self.peek().as_ref().unwrap().token_type.precedence();
               let expr = self.expression(precedence)?;
               self.consume(TokenType::CloseParen)?;
               Ok(expr)
            },
            TokenType::Identifier => {
               let line_number = self.peek().as_ref().unwrap().line_number;
               Ok(Expr::Var(self.identifier()?, line_number))
            },
            _ => {
               let t = self.peek();
               bail!(error::error(t.as_ref().unwrap().line_number,
                                  format!("Expected an expression, found '{}'", t.as_ref().unwrap().lexeme),
                                  error::ErrorType::SyntaxError))
            }
         }
      }
   }

   fn consume(&mut self, token_type: TokenType) -> Result<&Option<Token>> {
      if self.check(&token_type) {
         return Ok(self.advance());
      }
      bail!(error::error(self.peek().as_ref().unwrap().line_number,
                        format!("Expected '{}', found '{}'", token_type, self.peek().as_ref().unwrap().token_type),
                        error::ErrorType::SyntaxError))
   }

   fn match_token(&mut self, token_type: TokenType) -> bool {
      if self.check(&token_type) {
         self.advance();
         return true;
      }
      false
   }

   fn previous(&mut self) -> &mut Option<Token> {
      &mut self.tokens[self.current - 1]
   }

   fn check(&mut self, token_type: &TokenType) -> bool {
      if self.at_end() {
         return false;
      }
      &self.peek().as_ref().unwrap().token_type == token_type
   }

   fn advance(&mut self) -> &Option<Token> {
      let t = &self.tokens[self.current];
      self.current += 1;
      return t;
   }

   fn peek(&mut self) -> &Option<Token> {
      &self.tokens[self.current]
   }

   fn at_end(&mut self) -> bool {
      self.peek().as_ref().unwrap().token_type == TokenType::EOF
   }

   fn match_binary_op(&mut self) -> bool {
      if self.at_end() {
         return false;
      }
      if TokenType::BINARY_OPS.contains(&self.peek().as_ref().unwrap().token_type) {
         self.advance();
         return true;
      }
      false
   }

   fn match_unary_op(&mut self) -> bool {
      if self.at_end() {
         return false;
      }
      if TokenType::UNARY_OPS.contains(&self.peek().as_ref().unwrap().token_type) {
         self.advance();
         return true;
      }
      false
   }

   fn match_assignment_op(&mut self) -> bool {
      if self.at_end() {
         return false;
      }
      if TokenType::ASSIGNMENT_OPS.contains(&self.peek().as_ref().unwrap().token_type) {
         self.advance();
         return true;
      }
      false
   }
}