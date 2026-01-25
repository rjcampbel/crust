use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
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

pub enum ErrorType {
   InvalidToken,
   InvalidIdentifier,
}

pub fn error(line: usize, msg: String, err_type: ErrorType) -> LexError {
   match err_type {
      ErrorType::InvalidIdentifier => LexError::InvalidIdentifier { line, msg },
      ErrorType::InvalidToken => LexError::InvalidToken { line, msg }
   }
}
