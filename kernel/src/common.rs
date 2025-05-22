use thiserror::Error;

#[derive(Error, Debug)]
pub enum MimikoError {
    #[error("KERNEL | Lexer error: {0}")]
    LexerError(String),
    #[error("KERNEL | Parser error: {0}")]
    ParserError(String),
}
