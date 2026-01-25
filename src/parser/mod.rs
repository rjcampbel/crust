pub mod ast;
pub mod ast_printer;

use anyhow::{bail, ensure, Result};
use crate::lexer::token::{Token, TokenType};
use ast::*;
use num::traits::FromPrimitive;
use crate::error;

#[derive(PartialEq, PartialOrd, Clone, Copy, FromPrimitive)]
#[repr(u8)]
enum Precedence {
   None,
   Assignment,
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

pub fn parse(tokens: Vec<Token>, print_ast: bool) -> Result<AST> {
   let mut parser = Parser::new(tokens);
   let ast = parser.parse()?;
   if print_ast {
      ast_printer::print_ast(&ast);
   }
   Ok(ast)
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
      TokenType::GreaterOrEqual];

   fn to_binary_op(self) -> BinaryOp {
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

   fn to_unary_op(self) -> UnaryOp {
      match self {
         TokenType::Dash => UnaryOp::Negate,
         TokenType::Tilde => UnaryOp::Complement,
         TokenType::Bang => UnaryOp::Not,
         _ => unreachable!()
      }
   }
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
      let body = self.body()?;
      self.consume(TokenType::CloseBrace)?;
      Ok(Program::FunctionDefinition(FunctionDefinition::Function(name, body)))
   }

   fn body(&mut self) -> Result<Vec<BlockItem>> {
      let mut body = Vec::new();
      while !self.at_end() && self.peek().token_type != TokenType::CloseBrace {
         let block_item = self.block_item()?;
         body.push(block_item);
      }
      Ok(body)
   }

   fn block_item(&mut self) -> Result<BlockItem> {
      match self.peek().token_type {
         TokenType::Int => {
            self.declaration()
         },
         _ => self.statement()
      }
   }

   fn declaration(&mut self) -> Result<BlockItem> {
      self.consume(TokenType::Int)?;
      let name = self.identifier()?;
      let expr = if self.peek().token_type == TokenType::Equal {
         self.advance();
         Some(self.expression(Precedence::None)?)
      } else {
         None
      };
      self.consume(TokenType::Semicolon)?;
      Ok(BlockItem::Decl(Decl::Decl(name, expr)))
   }

   fn identifier(&mut self) -> Result<String> {
      match self.peek().token_type {
         TokenType::Identifier => {
            self.advance();
            Ok(self.previous().lexeme.clone())
         },
         _ => {
            let t = self.peek();
            bail!(error::error(t.line_number, format!("Expected an identifier, found '{}'", t.lexeme), error::ErrorType::SyntaxError))
         }
      }
   }

   fn statement(&mut self) -> Result<BlockItem> {
      if self.peek().token_type == TokenType::Return {
         self.advance();
         let expr = self.expression(Precedence::None)?;
         self.consume(TokenType::Semicolon)?;
         return Ok(BlockItem::Stmt(Stmt::Return(expr)))
      } else if self.peek().token_type == TokenType::Semicolon {
         self.advance();
         return Ok(BlockItem::Stmt(Stmt::Null));
      } else {
         let expr = self.expression(Precedence::None)?;
         self.consume(TokenType::Semicolon)?;
         return Ok(BlockItem::Stmt(Stmt::Expression(expr)));
      }
   }

   fn expression(&mut self, min_prec: Precedence) -> Result<Expr> {
      let mut left: Expr = self.factor()?;

      while self.peek().token_type.precedence() >= min_prec && (self.match_binary_op() || self.match_token(TokenType::Equal)) {
         if self.previous().token_type == TokenType::Equal {
            let prec = self.previous().token_type.precedence();
            let right = self.expression(prec)?;
            left = Expr::Assignment(Box::new(left.clone()), Box::new(right));
         } else {
            let operator_type = self.previous().token_type.clone();
            let next_prec = operator_type.precedence().increment();
            let binary_op = operator_type.to_binary_op();
            let right = self.expression(next_prec)?;
            left = Expr::BinaryOp {
               operator: binary_op,
               left: Box::new(left.clone()),
               right: Box::new(right)
            };
         }
      }
      Ok(left)
   }

   fn unary(&mut self) -> Result<Expr> {
      let operator_type = self.previous().token_type.clone();
      let unary_op = operator_type.to_unary_op();
      let expr = self.factor()?;
      Ok(Expr::UnaryOp {
         operator: unary_op,
         expr: Box::new(expr),
      })
   }

   fn factor(&mut self) -> Result<Expr> {
      if self.match_unary_op() {
         return self.unary()
      } else {
         match self.peek().token_type {
            TokenType::Integer(i) => {
               self.advance();
               Ok(Expr::Integer(i))
            },
            TokenType::OpenParen => {
               self.advance();
               let precedence = self.peek().token_type.precedence();
               let expr = self.expression(precedence)?;
               self.consume(TokenType::CloseParen)?;
               Ok(expr)
            },
            TokenType::Identifier => {
               Ok(Expr::Var(self.identifier()?))
            },
            _ => {
               let t = self.peek();
               bail!(error::error(t.line_number, format!("Expected an expression, found '{}'", t.lexeme), error::ErrorType::SyntaxError))
            }
         }
      }
   }

   fn consume(&mut self, token_type: TokenType) -> Result<&Token> {
      if self.check(&token_type) {
         return Ok(self.advance());
      }
      bail!(error::error(self.peek().line_number, format!("Expected '{}', found '{}'", token_type, self.peek().token_type), error::ErrorType::SyntaxError))
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

   fn match_binary_op(&mut self) -> bool {
      if self.at_end() {
         return false;
      }
      if TokenType::BINARY_OPS.contains(&self.peek().token_type) {
         self.advance();
         return true;
      }
      false
   }

   fn match_unary_op(&mut self) -> bool {
      if self.at_end() {
         return false;
      }
      if TokenType::UNARY_OPS.contains(&self.peek().token_type) {
         self.advance();
         return true;
      }
      false
   }
}