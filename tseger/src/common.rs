use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsegerError {
    #[error("TSEGER | Lexer error: {0}")]
    LexerError(String),
    #[error("TSEGER | Parser error: {0}")]
    ParserError(String),
    #[error("TSEGER | DFA error: {0}")]
    DFAError(String),
}
