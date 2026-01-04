#[derive(Debug, PartialEq, Clone)]
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
    pub token_type: TokenType,
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
}