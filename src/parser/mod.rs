use anyhow::Result;
use crate::Token;

pub fn parse(tokens: &Vec<Token>) -> Result<()> {
   let parser = Parser::new(tokens);
   parser.parse()?;
   Ok(())
}

struct Parser<'a> {
   tokens: &'a Vec<Token>
}

impl<'a> Parser<'a> {
   fn new(tokens: &'a Vec<Token>) -> Self {
      Self {
         tokens
      }
   }

   fn parse(&self) -> Result<()> {
      Ok(())
   }
}