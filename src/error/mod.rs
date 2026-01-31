use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
   #[error("[line {}] Error at '{}': Invalid Token", line, msg)]
   InvalidToken {
      line: usize,
      msg: String,
   },

   #[error("[line {}] Error at '{}': Invalid Identifier", line, msg)]
   InvalidIdentifier {
      line: usize,
      msg: String
   },

   #[error("[line {}] Syntax Error: {}", line, msg)]
   SyntaxError {
      line: usize,
      msg: String,
   },

   #[error("[line {}] Semantic Error: {}", line, msg)]
   SemanticError {
      line: usize,
      msg: String,
   }
}

pub enum ErrorType {
   InvalidToken,
   InvalidIdentifier,
   SyntaxError,
   SemanticError,
}

pub fn error(line: usize, msg: String, err_type: ErrorType) -> Error {
   match err_type {
      ErrorType::InvalidIdentifier => Error::InvalidIdentifier { line, msg },
      ErrorType::InvalidToken => Error::InvalidToken { line, msg },
      ErrorType::SyntaxError => Error::SyntaxError { line, msg },
      ErrorType::SemanticError => Error::SemanticError { line, msg },
   }
}
