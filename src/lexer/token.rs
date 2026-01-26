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
   Plus,
   DoublePlus,
   Star,
   Slash,
   Percent,
   Ampersand,
   DoubleAmpersand,
   Pipe,
   DoublePipe,
   Caret,
   Less,
   LessOrEqual,
   DoubleLess,
   Greater,
   GreaterOrEqual,
   DoubleGreater,
   Bang,
   DoubleEqual,
   BangEqual,
   Equal,
   PlusEqual,
   MinusEqual,
   StarEqual,
   SlashEqual,
   PercentEqual,
   AndEqual,
   OrEqual,
   XorEqual,
   LeftShiftEqual,
   RightShiftEqual,

   // Keywords
   Int,
   Void,
   Return,

   // Constands/Identifiers
   Integer(i64),
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
         TokenType::Plus => write!(f, "+"),
         TokenType::DoublePlus => write!(f, "++"),
         TokenType::Star => write!(f, "*"),
         TokenType::Slash => write!(f, "/"),
         TokenType::Percent => write!(f, "%"),
         TokenType::Ampersand => write!(f, "&"),
         TokenType::DoubleAmpersand => write!(f, "&&"),
         TokenType::Pipe => write!(f, "|"),
         TokenType::DoublePipe => write!(f, "||"),
         TokenType::Caret => write!(f, "^"),
         TokenType::Less => write!(f, "<"),
         TokenType::LessOrEqual => write!(f, "<="),
         TokenType::Greater => write!(f, ">"),
         TokenType::GreaterOrEqual => write!(f, ">"),
         TokenType::DoubleLess => write!(f, "<<"),
         TokenType::DoubleGreater => write!(f, ">>"),
         TokenType::Bang => write!(f, "!"),
         TokenType::DoubleEqual => write!(f, "=="),
         TokenType::BangEqual => write!(f, "!="),
         TokenType::Int => write!(f, "int"),
         TokenType::Void => write!(f, "void"),
         TokenType::Return => write!(f, "return"),
         TokenType::Integer(i) => write!(f, "{}", i),
         TokenType::Identifier => write!(f, "identifier"),
         TokenType::Equal => write!(f, "="),
         TokenType::PlusEqual => write!(f, "+="),
         TokenType::MinusEqual => write!(f, "-="),
         TokenType::StarEqual => write!(f, "*="),
         TokenType::SlashEqual => write!(f, "/="),
         TokenType::PercentEqual => write!(f, "%="),
         TokenType::AndEqual => write!(f, "&="),
         TokenType::OrEqual => write!(f, "|="),
         TokenType::XorEqual => write!(f, "^="),
         TokenType::LeftShiftEqual => write!(f, "<<="),
         TokenType::RightShiftEqual => write!(f, ">>="),
         TokenType::EOF => write!(f, "EOF"),
      }
   }
}