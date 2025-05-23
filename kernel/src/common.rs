use logos::Lexer;
use std::ops::Range;
use thiserror::Error;

use crate::lexer::Token;

#[derive(Error, Debug)]
pub enum MimikoError {
    #[error("KERNEL | Lexer error: {0}")]
    LexerError(String),
    #[error("KERNEL | Unexpected token at {}, {}", .range.start, .range.end)]
    ParserUnexpectedToken { range: Range<usize> },
    #[error("KERNEL | Unexpected end of sequence at {}, {}", .range.start, .range.end)]
    ParserUnexpectedEndSequence { range: Range<usize> },
    #[error("KERNEL | Parser error: {0}")]
    ParserError(String),
}
