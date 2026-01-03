#[derive(Debug)]
pub enum TokenType {
    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,

    // Keywords
    Int,
    Void,
    Return,

    // Constands/Identifiers
    Integer(u64),
    Identifier(String),
    EOF,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line_number: usize
}

impl Token {
   pub fn new(token_type: TokenType, lexeme: String, line_number: usize) -> Self {
      Self {
         token_type,
         lexeme,
         line_number,
      }
   }

   // pub fn token_type(&self) -> &TokenType {
   //    &self.token_type
   // }

   // pub fn line_number(&self) -> usize {
   //    self.line_number
   // }
}