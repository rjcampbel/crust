pub mod token;

use anyhow::{bail, Result};
use std::fs;
use std::path::Path;
use token::{Token, TokenType};
use thiserror::Error;

pub fn lex(source: &Path, print_tokens: bool) -> Result<Vec<Token>> {
   let source  = fs::read_to_string(source)?.chars().collect();
   let mut lexer = Lexer::new(&source);
   lexer.lex()?;

   if print_tokens {
      for token in &lexer.tokens {
         println!("{:?}", token);
      }
   }

   Ok(lexer.tokens)
}

struct Lexer<'a> {
   source: &'a Vec<char>,
   tokens: Vec<Token>,
   start: usize,
   current: usize,
   line: usize,
}

#[derive(Error, Debug)]
enum LexError {
   #[error("[line {}] Error at '{}': Invalid Token", line, msg)]
   InvalidToken {
      line: usize,
      msg: String,
   },

   #[error("[line {}] Error at '{}': Invalid Identifier", line, msg)]
   InvalidIdentifier {
      line: usize,
      msg: String
   }
}

enum ErrorType {
   InvalidToken,
   InvalidIdentifier,
}

impl<'a> Lexer<'a> {
   fn new(source: &'a Vec<char>) -> Self {
      Self {
         source,
         tokens: Vec::new(),
         start: 0,
         current: 0,
         line: 1,
      }
   }

   fn lex(&mut self) -> Result<()> {
      while !self.at_end() {
         self.start = self.current;
         self.scan_token()?;
      }

      let token = Token::new(TokenType::EOF, String::from(""), self.line);
      self.tokens.push(token);

      Ok(())
   }

   fn scan_token(&mut self) -> Result<()> {
      let c = self.advance();
      match c {
         '(' => self.add_token(TokenType::OpenParen),
         ')' => self.add_token(TokenType::CloseParen),
         '{' => self.add_token(TokenType::OpenBrace),
         '}' => self.add_token(TokenType::CloseBrace),
         ';' => self.add_token(TokenType::Semicolon),
         '~' => self.add_token(TokenType::Tilde),
         '-' => {
            if !self.at_end() && self.peek() == '-' {
               self.advance();
               self.add_token(TokenType::DoubleDash);
            } else {
               self.add_token(TokenType::Dash);
            }
         },
         '+' => self.add_token(TokenType::Plus),
         '*' => self.add_token(TokenType::Star),
         '/' => self.add_token(TokenType::Slash),
         '%' => self.add_token(TokenType::Percent),
         '&' => {
            if !self.at_end() && self.peek() == '&' {
               self.advance();
               self.add_token(TokenType::DoubleAmpersand);
            } else {
               self.add_token(TokenType::Ampersand);
            }
         },
         '|' => {
            if !self.at_end() && self.peek() == '|' {
               self.advance();
               self.add_token(TokenType::DoublePipe);
            } else {
               self.add_token(TokenType::Pipe);
            }
         },
         '^' => self.add_token(TokenType::Caret),
         '<' => {
            if !self.at_end() && self.peek() == '<' {
               self.advance();
               self.add_token(TokenType::DoubleLess);
            } else if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::LessOrEqual);
            } else {
               self.add_token(TokenType::Less);
            }
         },
         '>' => {
            if !self.at_end() && self.peek() == '>' {
               self.advance();
               self.add_token(TokenType::DoubleGreater);
            } else if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::GreaterOrEqual);
            } else {
               self.add_token(TokenType::Greater);
            }
         },
         '!' => {
            if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::BangEqual);
            } else {
               self.add_token(TokenType::Bang);
            }
         }
         '=' => {
            if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::DoubleEqual);
            } else {
               self.add_token(TokenType::Equal);
            }
         }
         '\n'=> self.line += 1,
         _ if c.is_whitespace() => (),
         _ if c.is_digit(10) => self.number()?,
         _ if is_alpha(c) => self.identifier()?,
         _ => bail!(error(self.line, String::from(c), ErrorType::InvalidToken))
      };

      Ok(())
   }

   fn add_token(&mut self, token_type: TokenType) {
      let lexeme = self.lexeme();
      let token = Token::new(token_type, lexeme, self.line);
      self.tokens.push(token);
   }

   fn advance(&mut self) -> char {
      let c = self.source[self.current];
      self.current += 1;
      c
   }

   fn peek(&self) -> char {
      self.source[self.current]
   }

   fn at_end(&self) -> bool {
      self.current >= self.source.len()
   }

   fn lexeme(&self) -> String {
      self.source[self.start..self.current].iter().collect()
   }

   fn number(&mut self) -> Result<()> {
      while !self.at_end() && is_digit(self.peek()) {
         self.advance();
      }

      if is_alpha(self.peek()) {
         while !self.at_end() && is_alpha(self.peek()) {
            self.advance();
         }
         bail!(error(self.line, self.lexeme(), ErrorType::InvalidIdentifier))
      }

      let token_string = self.lexeme();
      self.add_token(TokenType::Integer(token_string.parse::<i64>()?));

      Ok(())
   }

   fn identifier(&mut self) -> Result<()> {
      while !self.at_end() && is_alpha(self.peek()) {
         self.advance();
      }

      let token_string = self.lexeme();
      match token_string.as_str() {
         "int"    => &self.add_token(TokenType::Int),
         "void"   => &self.add_token(TokenType::Void),
         "return" => &self.add_token(TokenType::Return),
         _        => &self.add_token(TokenType::Identifier)
      };

      Ok(())
   }
}

fn is_alpha(c: char) -> bool {
   c.is_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
   c.is_digit(10)
}

fn error(line: usize, msg: String, err_type: ErrorType) -> LexError {
   match err_type {
      ErrorType::InvalidIdentifier => LexError::InvalidIdentifier { line, msg },
      ErrorType::InvalidToken => LexError::InvalidToken { line, msg }
   }
}