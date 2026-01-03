mod token;

use anyhow::{anyhow, Result};
use std::path::Path;
use std::fs;
use token::{Token, TokenType};

pub fn lex(source: &Path) -> Result<()> {
   let source  = fs::read_to_string(source)?.chars().collect();
   let mut lexer = Lexer::new(&source);
   lexer.lex()?;
   Ok(())
}

struct Lexer<'a> {
   source: &'a Vec<char>,
   line: usize,
   tokens: Vec<Token>,
   start: usize,
   current: usize,
}

impl<'a> Lexer<'a> {
   fn new(source: &'a Vec<char>) -> Self {
      Self {
         source,
         line: 1,
         tokens: Vec::new(),
         start: 0,
         current: 0,
      }
   }

   fn lex(&mut self) -> Result<()> {
      while self.current < self.source.len() {
         self.start = self.current;
         let c = self.advance();
         match c {
            '(' => self.add_token(TokenType::OpenParen, String::from(c)),
            ')' => self.add_token(TokenType::CloseParen, String::from(c)),
            '{' => self.add_token(TokenType::OpenBrace, String::from(c)),
            '}' => self.add_token(TokenType::CloseBrace, String::from(c)),
            ';' => self.add_token(TokenType::Semicolon, String::from(c)),
            '\n'=> self.line += 1,
            _ if c.is_whitespace() => (),
            _ if c.is_digit(10) => {
               while self.current < self.source.len() && self.peek().is_digit(10) {
                  self.advance();
               }
               match self.peek() {
                  c => {
                     if c.is_alphabetic() || c == '_' {
                        return Err(anyhow!(String::from("Invalid identifier")));
                     }
                  }
               }
               let token_value = self.source[self.start..self.current].iter().collect::<String>();
               self.add_token(TokenType::Integer(token_value.parse::<u64>()?), token_value);
            },
            _ if c.is_alphabetic() || c == '_' => {
               while self.current < self.source.len() && (self.peek().is_alphabetic() || self.peek() == '_') {
                  self.advance();
               }
               let token_value = self.source[self.start..self.current].iter().collect::<String>();
               match token_value.as_str() {
                  "int" => &self.add_token(TokenType::Int, token_value),
                  "void" => &self.add_token(TokenType::Void, token_value),
                  "return" => &self.add_token(TokenType::Return, token_value),
                  _ => &self.add_token(TokenType::Identifier(token_value.clone()), token_value)
               };
            },
            _ => {
               return Err(anyhow!(String::from("Invalid Token")));
            }
         }
      }
      println!("{:?}", self.tokens);
      Ok(())
   }

   fn add_token(&mut self, token_type: TokenType, lexeme: String) {
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
}