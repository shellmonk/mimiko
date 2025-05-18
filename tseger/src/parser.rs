// TODO: Remove this when module is ready
#![allow(dead_code)]

use crate::common::TsegerError;
use crate::lexer::Position;
use crate::lexer::RegexAtom;

pub enum RegexNodeKind {
    ROOT,
}

pub struct RegexNode {
    kind: RegexNodeKind,
    children: Vec<Option<Box<RegexNode>>>,
}

impl RegexNode {
    fn new(kind: RegexNodeKind) -> Self {
        Self {
            kind,
            children: Vec::new(),
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<(RegexAtom, Position)>,
    ast: Box<RegexNode>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<(RegexAtom, Position)>) -> Self {
        Self {
            tokens,
            ast: Box::from(RegexNode::new(RegexNodeKind::ROOT)),
        }
    }

    pub fn parse(&self) -> Result<Option<Box<RegexNode>>, TsegerError> {
        Ok(Some(Box::from(RegexNode::new(RegexNodeKind::ROOT))))
    }
}
