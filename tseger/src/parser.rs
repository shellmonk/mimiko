// TODO: Remove this when module is ready
#![allow(dead_code)]

use crate::common::TsegerError;
use crate::lexer::Position;
use crate::lexer::RegexAtom;

pub struct Parser<'a> {
    tokens: &'a Vec<(RegexAtom, Position)>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<(RegexAtom, Position)>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<AST, TsegerError> {
        Ok(AST::new())
    }
}

pub struct AST {}

impl AST {
    fn new() -> Self {
        Self {}
    }
}
