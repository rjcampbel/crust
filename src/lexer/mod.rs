mod token;

use anyhow::{anyhow, Result};
use std::path::Path;
use std::fs;
use token::{Token, TokenType};

pub fn lex(source: &Path) -> Result<()> {
   let source  = fs::read_to_string(source)?;
   let mut lexer = Lexer::new(&source);
   lexer.lex()?;
   Ok(())
}

struct Lexer<'a> {
   source: &'a String,
   line: usize,
   tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
   fn new(source: &'a String) -> Self {
      Self {
         source,
         line: 1,
         tokens: Vec::new(),
      }
   }

   fn lex(&mut self) -> Result<()> {
      let mut chars = self.source.chars().peekable();
      while let Some(c) = chars.next() {
         match c {
            '(' => self.add_token(TokenType::OpenParen, c.to_string()),
            ')' => self.add_token(TokenType::CloseParen, c.to_string()),
            '{' => self.add_token(TokenType::OpenBrace, c.to_string()),
            '}' => self.add_token(TokenType::CloseBrace, c.to_string()),
            ';' => self.add_token(TokenType::Semicolon, c.to_string()),
            '\n'=> self.line += 1,
            _ if c.is_whitespace() => (),
            _ if c.is_digit(10) => {
               let mut token_value: String = String::from(c);
               while let Some(t) = chars.peek() {
                  if t.is_digit(10) {
                     token_value.push(chars.next().unwrap());
                  } else {
                     break;
                  }
               }
               if let Some(t) = chars.peek() {
                  if t.is_alphabetic() || *t == '_' {
                     return Err(anyhow!(String::from("Invalid identifier")));
                  }
               }
               self.add_token(TokenType::Integer(token_value.parse::<u64>()?), token_value);
            },
            _ if c.is_alphabetic() || c == '_' => {
               let mut token_value: String = String::from(c);
               while let Some(t) = chars.peek() {
                  if t.is_alphanumeric() || *t == '_' {
                     token_value.push(chars.next().unwrap());
                  } else {
                     break;
                  }
               }
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
      Ok(())
   }

   fn add_token(&mut self, token_type: TokenType, lexeme: String) {
      let token = Token::new(token_type, lexeme, self.line);
      self.tokens.push(token);
   }
}