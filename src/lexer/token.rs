use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Dash,
    DoubleDash,

    // Keywords
    Int,
    Void,
    Return,

    // Constands/Identifiers
    Integer(u64),
    Identifier,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_number: usize
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

impl fmt::Display for TokenType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         TokenType::OpenParen => write!(f, "("),
         TokenType::CloseParen => write!(f, ")"),
         TokenType::OpenBrace => write!(f, "{{"),
         TokenType::CloseBrace => write!(f, "}}"),
         TokenType::Semicolon => write!(f, ";"),
         TokenType::Tilde => write!(f, "~"),
         TokenType::Dash => write!(f, "-"),
         TokenType::DoubleDash => write!(f, "--"),
         TokenType::Int => write!(f, "int"),
         TokenType::Void => write!(f, "void"),
         TokenType::Return => write!(f, "return"),
         TokenType::Integer(i) => write!(f, "{}", i),
         TokenType::Identifier => write!(f, "identifier"),
         TokenType::EOF => write!(f, "EOF"),
      }
   }
}