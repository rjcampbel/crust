pub mod ast;
pub mod ast_printer;

use crate::error;
use crate::lexer::token::{Token, TokenType};
use crate::validator::symbol_table::SymbolTable;

use anyhow::{bail, Result};
use ast::*;
use num::traits::FromPrimitive;

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
   Level2,
   Level1
}

impl Precedence {
   fn increment(&self) -> Precedence {
      match FromPrimitive::from_u8(*self as u8 + 1) {
         Some(p) => p,
         _ => Precedence::None,
      }
   }
}

impl TokenType {
   fn precedence(&self) -> Precedence {
      match self {
         TokenType::DoublePlus => Precedence::Level1,
         TokenType::DoubleDash => Precedence::Level1,
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
      TokenType::Bang,
      TokenType::DoublePlus,
      TokenType::DoubleDash,];

   fn to_unary_op(&self) -> UnaryOp {
      match self {
         TokenType::Dash => UnaryOp::Negate,
         TokenType::Tilde => UnaryOp::Complement,
         TokenType::Bang => UnaryOp::Not,
         TokenType::DoublePlus => UnaryOp::PreIncrement,
         TokenType::DoubleDash => UnaryOp::PreDecrement,
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
      let ast = AST { program , symbol_table: SymbolTable::new() };
      if print_ast {
         ast_printer::print_ast(&ast);
      }

      Ok(ast)
   }

   fn program(&mut self) -> Result<Program> {
      let mut decls: Vec<Decl> = Vec::new();
      while !self.at_end() {
         let decl = self.declaration()?;
         decls.push(decl);
      }
      Ok(Program{ decls })
   }

   fn function_decl(&mut self, name: String, storage_class: Option<StorageClass>, line_number: usize) -> Result<FuncDecl> {
      self.consume(TokenType::OpenParen)?;
      let params = self.params()?;
      self.consume(TokenType::CloseParen)?;

      let block = if !self.match_token(TokenType::OpenBrace) {
         self.consume(TokenType::Semicolon)?;
         None
      } else {
         let block = self.block()?;
         self.consume(TokenType::CloseBrace)?;
         Some(block)
      };
      Ok(FuncDecl{ name, params, body: block, storage_class, line_number })
   }

   fn variable_decl(&mut self, name: String, storage_class: Option<StorageClass>,line_number: usize) -> Result<VarDecl> {
      let init = if !self.match_token(TokenType::Equal) {
         None
      } else {
         Some(self.expression(Precedence::None)?)
      };
      self.consume(TokenType::Semicolon)?;
      Ok(VarDecl{ name, init, storage_class, line_number })
   }

   fn param(&mut self) -> Result<String> {
      self.consume(TokenType::Int)?;
      Ok(self.identifier()?)
   }

   fn params(&mut self) -> Result<Vec<String>> {
      let mut params = Vec::new();
      if !self.match_token(TokenType::Void) {
         params.push(self.param()?);
         while self.match_token(TokenType::Comma) {
            params.push(self.param()?);
         }
      }
      Ok(params)
   }

   fn block(&mut self) -> Result<Block> {
      let mut items = Vec::new();
      while !self.at_end() && self.peek().as_ref().unwrap().token_type != TokenType::CloseBrace {
         let block_item = self.block_item()?;
         items.push(block_item);
      }
      Ok(Block{ items })
   }

   fn block_item(&mut self) -> Result<BlockItem> {
      match self.peek().as_ref().unwrap().token_type {
         TokenType::Int | TokenType::Static | TokenType::Extern => {
            Ok(BlockItem::Decl(self.declaration()?))
         },
         _ => Ok(BlockItem::Stmt(self.statement()?))
      }
   }

   fn type_and_storage_class(&mut self) -> Result<Option<StorageClass>> {
      let mut types = Vec::new();
      let mut storage_classes = Vec::new();
      while self.peek().as_ref().unwrap().token_type != TokenType::Identifier {
         if self.match_token(TokenType::Int) {
            types.push(TokenType::Int);
         } else if self.match_token(TokenType::Static) {
            storage_classes.push(StorageClass::Static);
         } else if self.match_token(TokenType::Extern) {
            storage_classes.push(StorageClass::Extern);
         } else {
            let t = self.peek();
            bail!(error::error(t.as_ref().unwrap().line_number,
                  format!("Expected a type or storage class, found '{}'", t.as_ref().unwrap().lexeme),
                  error::ErrorType::SyntaxError))
         }
      }
      if types.len() != 1 {
         bail!(error::error(self.peek().as_ref().unwrap().line_number,
               format!("Invalid type specifier"),
               error::ErrorType::SyntaxError))
      }
      if storage_classes.len() > 1 {
         bail!(error::error(self.peek().as_ref().unwrap().line_number,
               format!("Invalid storage class"),
               error::ErrorType::SyntaxError))
      }
      let storage_class = if storage_classes.len() == 1 {
         Some(storage_classes[0])
      } else {
         None
      };
      Ok(storage_class)
   }

   fn declaration(&mut self) -> Result<Decl> {
      let storage_class = self.type_and_storage_class()?;
      let name = self.identifier()?;
      let line_number = self.peek().as_ref().unwrap().line_number;
      let decl =
         if self.peek().as_ref().unwrap().token_type == TokenType::OpenParen {
            Ok(Decl::FuncDecl(self.function_decl(name, storage_class, line_number)?))
         } else {
            Ok(Decl::VarDecl(self.variable_decl(name, storage_class, line_number)?))
         };
      return decl;
   }

   fn identifier(&mut self) -> Result<String> {
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

   fn labels(&mut self) -> Result<Vec<Label>> {
      let mut labels = Vec::new();
      while self.peek().as_ref().unwrap().token_type == TokenType::Identifier
            && self.peek_n(1).as_ref().unwrap().token_type == TokenType::Colon {
         self.advance();
         let line_number = self.previous().as_ref().unwrap().line_number;
         let label = Label { name: self.peek().as_ref().unwrap().lexeme.clone(), line_number };
         self.advance();
         labels.push(label);
      }
      Ok(labels)
   }

   fn statement(&mut self) -> Result<Stmt> {
      let labels = self.labels()?;
      match self.peek().as_ref().unwrap().token_type {
         TokenType::Return => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            let expr = self.expression(Precedence::None)?;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Return(expr, labels, line_number))
         },
         TokenType::Semicolon => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            return Ok(Stmt::Null(labels, line_number));
         },
         TokenType::If => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
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
            return Ok(Stmt::If(expr, Box::new(then_stmt), else_stmt, labels, line_number));
         },
         TokenType::OpenBrace => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            let block = self.block()?;
            self.consume(TokenType::CloseBrace)?;
            return Ok(Stmt::Compound(block, labels, line_number));
         },
         TokenType::Break => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Break("".into(), labels, line_number));
         },
         TokenType::Continue => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Continue("".into(), labels, line_number));
         },
         TokenType::While => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            self.consume(TokenType::OpenParen)?;
            let condition = self.expression(Precedence::None)?;
            self.consume(TokenType::CloseParen)?;
            let body = self.statement()?;
            return Ok(Stmt::While(condition, Box::new(body), labels, line_number));
         },
         TokenType::Do => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            let body = self.statement()?;
            self.consume(TokenType::While)?;
            self.consume(TokenType::OpenParen)?;
            let condition = self.expression(Precedence::None)?;
            self.consume(TokenType::CloseParen)?;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::DoWhile(Box::new(body), condition, labels, line_number));
         },
         TokenType::For => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            self.consume(TokenType::OpenParen)?;
            let for_init = self.for_init()?;
            let condition = self.optional_expression(TokenType::Semicolon, Precedence::None)?;
            self.consume(TokenType::Semicolon)?;
            let post = self.optional_expression(TokenType::CloseParen, Precedence::None)?;
            self.consume(TokenType::CloseParen)?;
            let body = self.statement()?;
            Ok(Stmt::For(for_init, condition, post, Box::new(body), labels, line_number))
         },
         TokenType::Goto => {
            self.advance();
            let line_number = self.previous().as_ref().unwrap().line_number;
            self.consume(TokenType::Identifier)?;
            let label = self.previous().as_ref().unwrap().lexeme.clone();
            self.consume(TokenType::Semicolon)?;
            Ok(Stmt::Goto(label, labels, line_number))
         }
         _ => {
            let line_number = self.peek().as_ref().unwrap().line_number;
            let expr = self.expression(Precedence::None)?;
            self.consume(TokenType::Semicolon)?;
            return Ok(Stmt::Expression(expr, labels, line_number));
         }
      }
   }

   fn for_init(&mut self) -> Result<Option<ForInit>> {
      if !self.match_token(TokenType::Semicolon) {
         let next_token_type = &self.peek().as_ref().unwrap().token_type;
         if *next_token_type == TokenType::Int || *next_token_type == TokenType::Static || *next_token_type == TokenType::Extern {
            let storage_class = self.type_and_storage_class()?;
            let name = self.identifier()?;
            let line_number = self.peek().as_ref().unwrap().line_number;
            Ok(Some(ForInit::Decl(self.variable_decl(name, storage_class, line_number)?)))
         } else {
            let init = Some(ForInit::Expr(self.expression(Precedence::None)?));
            self.consume(TokenType::Semicolon)?;
            Ok(init)
         }
      } else {
         Ok(None)
      }
   }

   fn optional_expression(&mut self, delimiter: TokenType, min_prec: Precedence) -> Result<Option<Expr>> {
      if self.peek().as_ref().unwrap().token_type == delimiter {
         Ok(None)
      } else {
         Ok(Some(self.expression(min_prec)?))
      }
   }

   fn expression(&mut self, min_prec: Precedence) -> Result<Expr> {
      let mut left: Expr = self.factor()?;

      while self.peek().as_ref().unwrap().token_type.precedence() >= min_prec {
         if self.match_binary_op() {
            let next_prec = self.previous().as_ref().unwrap().token_type.precedence().increment();
            let binary_op = self.previous().as_ref().unwrap().token_type.to_binary_op();
            let right = self.expression(next_prec)?;
            left = Expr::BinaryOp(binary_op, Box::new(left), Box::new(right));
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

      if self.match_token(TokenType::DoublePlus) || self.match_token(TokenType::DoubleDash) {
         let line_number = self.previous().as_ref().unwrap().line_number;
         let op =
            if self.previous().take().unwrap().token_type == TokenType::DoublePlus {
               UnaryOp::PostIncrement
            } else {
               UnaryOp::PostDecrement
            };
         left = Expr::UnaryOp(op, Box::new(left), line_number);
      }
      Ok(left)
   }

   fn compound_assignment(&mut self, op: BinaryOp, left: Expr) -> Result<Expr> {
      let line_number = self.previous().as_ref().unwrap().line_number;
      let prec = self.previous().as_ref().unwrap().token_type.precedence();
      let right = Expr::BinaryOp(op, Box::new(left.clone()), Box::new(self.expression(prec)?));
      Ok(Expr::Assignment(Box::new(left), Box::new(right), line_number))
   }

   fn unary(&mut self) -> Result<Expr> {
      let unary_op = self.previous().as_ref().unwrap().token_type.to_unary_op();
      let line_number = self.previous().as_ref().unwrap().line_number;
      let expr = self.factor()?;
      Ok(Expr::UnaryOp(unary_op, Box::new(expr), line_number))
   }

   fn arg(&mut self) -> Result<Expr> {
      self.expression(Precedence::None)
   }

   fn args(&mut self) -> Result<Vec<Expr>> {
      let mut args = Vec::new();
      while self.peek().as_ref().unwrap().token_type != TokenType::CloseParen {
         args.push(self.arg()?);
         while self.match_token(TokenType::Comma) {
            args.push(self.arg()?);
         }
      }
      Ok(args)
   }

   fn factor(&mut self) -> Result<Expr> {
      let mut expr: Expr;
      if self.match_unary_op() {
         expr = self.unary()?;
      } else {
         expr =
            match self.peek().as_ref().unwrap().token_type {
               TokenType::Integer(i) => {
                  self.advance();
                  Expr::Integer(i)
               },
               TokenType::OpenParen => {
                  self.advance();
                  let expr = self.expression(Precedence::None)?;
                  self.consume(TokenType::CloseParen)?;
                  expr
               },
               TokenType::Identifier => {
                  let line_number = self.peek().as_ref().unwrap().line_number;
                  let name = self.identifier()?;
                  if self.match_token(TokenType::OpenParen) {
                     let args = self.args()?;
                     self.consume(TokenType::CloseParen)?;
                     Expr::FunctionCall(name, args, line_number)
                  } else {
                     Expr::Var(name, line_number)
                  }
               },
               _ => {
                  let t = self.peek();
                  bail!(error::error(t.as_ref().unwrap().line_number,
                                    format!("Expected an expression, found '{}'", t.as_ref().unwrap().lexeme),
                                    error::ErrorType::SyntaxError))
               }
            }
      }
      if self.match_token(TokenType::DoublePlus) || self.match_token(TokenType::DoubleDash) {
         let line_number = self.previous().as_ref().unwrap().line_number;
         let op =
            if self.previous().take().unwrap().token_type == TokenType::DoublePlus {
               UnaryOp::PostIncrement
            } else {
               UnaryOp::PostDecrement
            };
         expr = Expr::UnaryOp(op, Box::new(expr), line_number);
      }
      Ok(expr)
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

   fn peek_n(&mut self, n: usize) -> &Option<Token> {
      &self.tokens[self.current + n]
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