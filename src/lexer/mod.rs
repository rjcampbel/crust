pub mod token;

use crate::error;

use anyhow::{bail, Result};
use token::{Token, TokenType};

struct Lexer<'a> {
   source: &'a String,
   tokens: Vec<Option<Token>>,
   start: usize,
   current: usize,
   line: usize,
}

pub fn lex(source: &String, print_tokens: bool) -> Result<Vec<Option<Token>>> {
   let mut lexer = Lexer::new(source);
   lexer.lex(print_tokens)?;
   Ok(lexer.tokens)
}

impl<'a> Lexer<'a> {
   pub fn new(source: &'a String) -> Self {
      Self {
         source,
         tokens: Vec::new(),
         start: 0,
         current: 0,
         line: 1,
      }
   }

   pub fn lex(&mut self, print_tokens: bool) -> Result<()> {
      while !self.at_end() {
         self.start = self.current;
         self.scan_token()?;
      }

      let token = Token::new(TokenType::EOF, String::from(""), self.line);
      self.tokens.push(Some(token));

      if print_tokens {
         for token in &self.tokens {
            println!("{:?}", token);
         }
      }

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
            } else if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::MinusEqual);
            } else {
               self.add_token(TokenType::Dash);
            }
         },
         '+' => {
            if !self.at_end() && self.peek() == '+' {
               self.advance();
               self.add_token(TokenType::DoublePlus);
            } else if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::PlusEqual);
            } else {
               self.add_token(TokenType::Plus);
            }
         },
         '*' => {
            if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::StarEqual);
            } else {
               self.add_token(TokenType::Star);
            }
         }
         '/' => {
            if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::SlashEqual);
            } else {
               self.add_token(TokenType::Slash);
            }
         },
         '%' => {
            if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::PercentEqual);
            } else {
               self.add_token(TokenType::Percent);
            }
         },
         '&' => {
            if !self.at_end() && self.peek() == '&' {
               self.advance();
               self.add_token(TokenType::DoubleAmpersand);
            } else if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::AndEqual);
            } else {
               self.add_token(TokenType::Ampersand);
            }
         },
         '|' => {
            if !self.at_end() && self.peek() == '|' {
               self.advance();
               self.add_token(TokenType::DoublePipe);
            } else if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::OrEqual);
            } else {
               self.add_token(TokenType::Pipe);
            }
         },
         '^' => {
            if !self.at_end() && self.peek() == '=' {
               self.advance();
               self.add_token(TokenType::XorEqual);
            } else {
               self.add_token(TokenType::Caret);
            }
         }
         '<' => {
            if !self.at_end() && self.peek() == '<' {
               self.advance();
               if !self.at_end() && self.peek() == '=' {
                  self.advance();
                  self.add_token(TokenType::LeftShiftEqual);
               } else {
                  self.add_token(TokenType::DoubleLess);
               }
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
               if !self.at_end() && self.peek() == '=' {
                  self.advance();
                  self.add_token(TokenType::RightShiftEqual);
               } else {
                  self.add_token(TokenType::DoubleGreater);
               }
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
         },
         '?' => self.add_token(TokenType::Question),
         ':' => self.add_token(TokenType::Colon),
         ',' => self.add_token(TokenType::Comma),
         '\n'=> self.line += 1,
         _ if c.is_whitespace() => (),
         _ if c.is_digit(10) => self.number()?,
         _ if is_alpha(c) => self.identifier()?,
         _ => bail!(error::error(self.line, String::from(c), error::ErrorType::InvalidToken))
      };

      Ok(())
   }

   fn add_token(&mut self, token_type: TokenType) {
      let line_number = self.line;
      let lexeme = self.lexeme();
      let token = Token::new(token_type, lexeme.to_string(), line_number);
      self.tokens.push(Some(token));
   }

   fn advance(&mut self) -> char {
      let c = self.source.chars().nth(self.current);
      self.current += 1;
      c.unwrap()
   }

   fn peek(&self) -> char {
      self.source.chars().nth(self.current).unwrap()
   }

   fn at_end(&self) -> bool {
      self.current >= self.source.len()
   }

   fn lexeme(&self) -> &str {
      &self.source.as_str()[self.start..self.current]
   }

   fn number(&mut self) -> Result<()> {
      while !self.at_end() && is_digit(self.peek()) {
         self.advance();
      }

      if is_alpha(self.peek()) {
         while !self.at_end() && (is_alpha(self.peek()) || is_digit(self.peek())) {
            self.advance();
         }
         bail!(error::error(self.line, self.lexeme().to_string(), error::ErrorType::InvalidIdentifier))
      }

      let token_string = self.lexeme();
      self.add_token(TokenType::Integer(token_string.parse::<i64>()?));

      Ok(())
   }

   fn identifier(&mut self) -> Result<()> {
      while !self.at_end() && (is_alpha(self.peek()) || is_digit(self.peek())) {
         self.advance();
      }

      let token_string = self.lexeme();

      if let Some(t  @ _) = to_keyword(&token_string) {
         self.add_token(t);
      } else {
         self.add_token(TokenType::Identifier);
      }

      Ok(())
   }
}

fn is_alpha(c: char) -> bool {
   c.is_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
   c.is_digit(10)
}

fn to_keyword(identifier: &str) -> Option<TokenType> {
   match identifier {
      "int" => Some(TokenType::Int),
      "void" => Some(TokenType::Void),
      "return" => Some(TokenType::Return),
      "if" => Some(TokenType::If),
      "else" => Some(TokenType::Else),
      "do" => Some(TokenType::Do),
      "while" => Some(TokenType::While),
      "for" => Some(TokenType::For),
      "break" => Some(TokenType::Break),
      "continue" => Some(TokenType::Continue),
      _ => None
   }
}