use crate::lexer::RegexAtom;

pub fn parse(tokens: Vec<RegexAtom>) -> AST {
    AST::new()
}

pub struct AST {}

impl AST {
    fn new() -> Self {
        Self {}
    }
}
