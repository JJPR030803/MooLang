use std::fmt;

/// # LexerError
/// This enum represents all the errors corresponding to the Lexer struct.
/// ## Currently 2 variants
/// - FileReadError(String)
/// - TokenizationError(String)
#[derive(Debug)]
pub enum LexerError {
    FileReadError(String),
    TokenizationError(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::FileReadError(msg) => write!(f, "File read error: {}", msg),
            LexerError::TokenizationError(msg) => write!(f, "Tokenization error: {}", msg),
        }
    }
}

impl std::error::Error for LexerError {}