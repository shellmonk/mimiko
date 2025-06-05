// TODO: Remove this when module is ready
#![allow(dead_code)]

use std::iter::Peekable;

use crate::common::TsegerError;
use crate::lexer::Position;
use crate::lexer::RegexAtom;
use crate::lexer::WhitespaceKind;

type ParserResult = Result<RegexAST, TsegerError>;

#[derive(Debug, PartialEq, Eq)]
pub enum RegexAST {
    Literal(char),
    Concat(Vec<RegexAST>),
    Alteration(Vec<RegexAST>),
    Group(Box<RegexAST>),
    CharClass(String),
    BracketExpression {
        negated: bool,
        items: Vec<BracketExpressionItem>,
    },
    Variable(String),
    Repetition {
        node: Box<RegexAST>,
        min: u32,
        max: Option<u32>,
    },
    Star(Box<RegexAST>),
    Plus(Box<RegexAST>),
    Question(Box<RegexAST>),
    Dot,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BracketExpressionItem {
    Char(char),
    Range(char, char),
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_expression(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = RegexAtom>>,
    ) -> Result<RegexAST, TsegerError> {
        self.parse_alteration(tokens)
    }

    fn parse_alteration(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = RegexAtom>>,
    ) -> Result<RegexAST, TsegerError> {
        let mut branches = vec![self.parse_concatenation(tokens)?];

        //let mut node = self.parse_concatenation(tokens)?;
        while let Some(RegexAtom::Or) = tokens.peek() {
            tokens.next();
            branches.push(self.parse_concatenation(tokens)?);
            //node = RegexAST::Alteration(Box::new(node), Box::new(rhs));
        }

        if branches.len() == 1 {
            Ok(branches.into_iter().next().unwrap())
        } else {
            Ok(RegexAST::Alteration(branches))
        }
    }

    fn parse_concatenation(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = RegexAtom>>,
    ) -> Result<RegexAST, TsegerError> {
        let mut nodes: Vec<RegexAST> = vec![];

        while let Some(token) = tokens.peek() {
            match token {
                RegexAtom::RParen | RegexAtom::Or => break,
                _ => nodes.push(self.parse_repetition(tokens)?),
            }
        }

        match nodes.len() {
            0 => Err(TsegerError::ParserError(format!("Expected expression"))),
            1 => Ok(nodes.remove(0)),
            _ => Ok(RegexAST::Concat(nodes)),
        }
    }

    fn parse_repetition(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = RegexAtom>>,
    ) -> Result<RegexAST, TsegerError> {
        let mut node = self.parse_atom(tokens)?;

        loop {
            match tokens.peek() {
                Some(RegexAtom::QuantKleene) => {
                    tokens.next();
                    node = RegexAST::Star(Box::new(node));
                }
                Some(RegexAtom::QuantPlus) => {
                    tokens.next();
                    node = RegexAST::Plus(Box::new(node));
                }
                Some(RegexAtom::QuantOptional) => {
                    tokens.next();
                    node = RegexAST::Question(Box::new(node));
                }
                Some(RegexAtom::Repetition(from, to)) => {
                    node = RegexAST::Repetition {
                        min: from.unwrap_or(0),
                        max: *to,
                        node: Box::new(node),
                    }
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_atom(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = RegexAtom>>,
    ) -> Result<RegexAST, TsegerError> {
        match tokens.next() {
            Some(RegexAtom::Literal(c)) => Ok(RegexAST::Literal(c)),
            Some(RegexAtom::QuantWildcard) => Ok(RegexAST::Dot),
            Some(RegexAtom::Whitespace(kind)) => Ok(RegexAST::Literal(match kind {
                WhitespaceKind::Space => ' ',
                WhitespaceKind::Tab => '\t',
                WhitespaceKind::NewLine => '\n',
                WhitespaceKind::CR => '\r',
            })),
            Some(RegexAtom::LParen) => self.parse_expression(tokens),
            _ => Err(TsegerError::ParserError(format!("unexpected token"))),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_alteration_1() {
        let lexer = Lexer::new("a|b|c");
        let parser = Parser {};

        let result = parser
            .parse_expression(&mut lexer.lex().unwrap().into_iter().map(|e| e.0).peekable())
            .unwrap();

        assert_eq!(
            result,
            RegexAST::Alteration(vec![
                RegexAST::Literal('a'),
                RegexAST::Literal('b'),
                RegexAST::Literal('c')
            ])
        );
    }
}
