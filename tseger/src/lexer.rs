// TODO: Remove this when module is ready
#![allow(dead_code)]

use crate::common::TsegerError;
use std::iter::Peekable;
/*
* Regex atoms:
*
* - abc123          literals
* - [a-zA-Z0-9]     ranges
* - .               {1,1} wildcard quantifier
* - ?               {0,1} optional quantifier
* - *               {0,} Kleene star quantifier
* - +               {1,} plus quantifier
* - |               logical OR
* - a{,5}|((b|c)*)  groupings and nested expressions
* - \* \\           escaping special characters
* - \t \r \n        special whitespace characters
* - ^               negation
* - \p{digits}      character classes (TBD) (digits, emojis, uppercase_ascii, cyrilic, etc.)
* - \xFFFF          Unicode scalar values support
* - \x{FFF0,FFFF}   Unicode scalar value ranges
* - #{names}        Variables
*/

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RegexAtom {
    Literal(char),     // ascii - abc123; unicode scalar values - \xFFFF
    Range(char, char), // ascii - [a-zA-Z0-9]; unicode ranges - \x{FFF0,FFFF}
    QuantWildcard,     // . {1,1}
    QuantOptional,     // ? {0,1}
    QuantKleene,       // * {0,}
    QuantPlus,         // + {1,}
    Or,                // | logical OR
    LParen,            // (
    RParen,            // )
    BracketExpressions {
        negated: bool,
        ranges: Vec<BracketExpression>,
    }, // [abc-Z0-9123]
    Repetition(Option<u32>, Option<u32>), // {69,420}
    Whitespace(WhitespaceKind), // \t \r \n
    CharClass(String), // \p{digits}
    Variable(String),  // #{var}
    EOF,
}

pub type PositionedAtom = (RegexAtom, Position);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum BracketExpression {
    Single(PositionedAtom),
    Ranged(PositionedAtom),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WhitespaceKind {
    NewLine,
    Tab,
    CR,
    Space,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    start: usize,
    end: usize,
}

pub struct Lexer<'a> {
    rx: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(regex: &'a str) -> Self {
        Self { rx: regex }
    }

    pub fn lex(&self) -> Result<Vec<PositionedAtom>, TsegerError> {
        let mut tokens = Vec::new();

        let mut iter = self.rx.chars().enumerate().into_iter().peekable();

        while let Some((i, c)) = iter.next() {
            match c {
                '(' => tokens.push((RegexAtom::LParen, Position { start: i, end: i })),
                ')' => tokens.push((RegexAtom::RParen, Position { start: i, end: i })),
                '.' => tokens.push((RegexAtom::QuantWildcard, Position { start: i, end: i })),
                '*' => tokens.push((RegexAtom::QuantKleene, Position { start: i, end: i })),
                '?' => tokens.push((RegexAtom::QuantOptional, Position { start: i, end: i })),
                '+' => tokens.push((RegexAtom::QuantPlus, Position { start: i, end: i })),
                '|' => tokens.push((RegexAtom::Or, Position { start: i, end: i })),
                ' ' => tokens.push((
                    RegexAtom::Whitespace(WhitespaceKind::Space),
                    Position { start: i, end: i },
                )),
                '\n' => tokens.push((
                    RegexAtom::Whitespace(WhitespaceKind::NewLine),
                    Position { start: i, end: i },
                )),

                '\t' => tokens.push((
                    RegexAtom::Whitespace(WhitespaceKind::Tab),
                    Position { start: i, end: i },
                )),
                '\r' => tokens.push((
                    RegexAtom::Whitespace(WhitespaceKind::CR),
                    Position { start: i, end: i },
                )),

                '{' => tokens.push(self.lex_repetitions(&mut iter)?),
                // TODO: This doesn't look really safe
                '[' => tokens.push(self.lex_bracket_expression(&mut iter)?),
                '#' => tokens.push(self.lex_variables(&mut iter)?),
                '\\' => match iter.next() {
                    None => {
                        return Err(TsegerError::LexerError(
                            "Unescaped slash at the end of the regex".to_string(),
                        ));
                    }
                    Some(following) => {
                        let fc = following.1;
                        match fc {
                            fc if vec![
                                '(', ')', '{', '}', '[', ']', '.', '*', '?', '+', '^', '|', '\\',
                                '#',
                            ]
                            .contains(&fc) =>
                            {
                                tokens.push((
                                    RegexAtom::Literal(fc),
                                    Position {
                                        start: i,
                                        end: i + 1,
                                    },
                                ))
                            }
                            'n' => tokens.push((
                                RegexAtom::Whitespace(WhitespaceKind::NewLine),
                                Position {
                                    start: i,
                                    end: i + 1,
                                },
                            )),
                            'r' => tokens.push((
                                RegexAtom::Whitespace(WhitespaceKind::CR),
                                Position {
                                    start: i,
                                    end: i + 1,
                                },
                            )),
                            't' => tokens.push((
                                RegexAtom::Whitespace(WhitespaceKind::Tab),
                                Position {
                                    start: i,
                                    end: i + 1,
                                },
                            )),
                            'x' => tokens.push(self.lex_unicode(&mut iter)?),
                            'p' => tokens.push(self.lex_char_classes(&mut iter)?),
                            _ => todo!("This needs to be handled, unknown character after slash"),
                        }
                    }
                },
                c if c.is_alphanumeric() => {
                    tokens.push((RegexAtom::Literal(c), Position { start: i, end: i }))
                }
                _ => todo!("Case {} at position {} not covered yet", c, i),
            }
        }

        Ok(tokens)
    }

    fn lex_repetitions<I>(&self, iter: &mut Peekable<I>) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut end;

        let mut start_str = String::new();
        let mut end_str = String::new();

        let mut first = true;

        let start = match iter.peek() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Unexpected end of sequence"
                )));
            }
            Some((i, _)) => *i,
        };

        end = start;

        while let Some((i, c)) = iter.next() {
            end = i;
            match c {
                c if c.is_numeric() => {
                    if first {
                        start_str.push(c)
                    } else {
                        end_str.push(c)
                    }
                }
                ',' => first = false,
                '}' => break,
                _ => {
                    return Err(TsegerError::LexerError(format!(
                        "Unexpected character '{}' at {}",
                        c, i
                    )));
                }
            }
        }

        let start_range = match start_str.is_empty() {
            true => Option::None,
            false => Option::Some(match u32::from_str_radix(start_str.as_str(), 10) {
                Ok(n) => n,
                Err(_) => {
                    return Err(TsegerError::LexerError(format!(
                        "Error while parsing repetition at {}",
                        end
                    )));
                }
            }),
        };

        let end_range = match end_str.is_empty() {
            true => Option::None,
            false => Option::Some(match u32::from_str_radix(end_str.as_str(), 10) {
                Ok(n) => n,
                Err(_) => {
                    return Err(TsegerError::LexerError(format!(
                        "Error while parsing repetition at {}",
                        end
                    )));
                }
            }),
        };

        Ok((
            RegexAtom::Repetition(start_range, end_range),
            Position { start, end },
        ))
    }

    // TODO: This method signature hurts to watch, there's probably a way in Rust to make it a bit more
    // elegant
    fn lex_bracket_expression<I>(
        &self,
        iter: &mut Peekable<I>,
    ) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut ranges = Vec::new();
        let mut negated = false;

        let mut start = match iter.peek() {
            // TODO: This requires a nicer error message, with position and so on
            None => return Err(TsegerError::LexerError(format!("Unexpected end of range"))),
            Some((i, _)) => *i,
        };

        let mut end = start;

        match iter.peek() {
            Some((_, c)) => {
                if *c == '^' {
                    negated = true;
                    iter.next();
                    start = start + 1;
                }
            }
            _ => (),
        }

        while let Some((i, c)) = iter.next_if(|(_, c)| *c != ']') {
            end = i;
            start = i;
            // TODO: Refactor this, looks ugly
            let next = iter.peek().unwrap();

            if next.1 == '-' {
                let range_start = c;

                iter.next();
                end = end + 1;

                let range_end = match iter.next() {
                    None => {
                        return Err(TsegerError::LexerError(format!(
                            "Unexpected end of range at {}",
                            end
                        )));
                    }
                    Some((_, local_end)) => local_end,
                };

                end = end + 1;

                ranges.push(BracketExpression::Ranged((
                    RegexAtom::Range(range_start, range_end),
                    Position { start, end },
                )));
            } else {
                ranges.push(BracketExpression::Single((
                    RegexAtom::Literal(c),
                    Position { start: end, end },
                )));
            }
        }

        match iter.next() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Unexpected end of range at {}",
                    end
                )));
            }
            Some((i, c)) => {
                if c != ']' {
                    return Err(TsegerError::LexerError(format!(
                        "Unexpected end of range at {}, missing ']'",
                        i
                    )));
                }
            }
        }

        Ok((
            RegexAtom::BracketExpressions { ranges, negated },
            Position { start, end },
        ))
    }

    fn lex_unicode<I>(&self, iter: &mut Peekable<I>) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        // TODO: Refactor this, it really looks like shit
        match iter.peek() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Invalid end of the expression, \\x is the last char"
                )));
            }
            Some((i, c)) => {
                if !c.is_ascii_hexdigit() && (*c != '{') {
                    return Err(TsegerError::LexerError(format!(
                        "Invalid token '{}' after \\x at position: {}",
                        c, i
                    )));
                }

                if c.is_ascii_hexdigit() {
                    return Ok(self.lex_unicode_char(iter)?);
                } else {
                    return Ok(self.lex_unicode_range(iter)?);
                }
            }
        }
    }

    fn lex_unicode_char<I>(&self, iter: &mut Peekable<I>) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut uchr = String::new();
        let start: usize;
        let mut end: usize = 0;

        match iter.peek() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Unexpected end while parsing unicode character"
                )));
            }
            Some((i, _)) => start = *i - 2,
        }

        while let Some((i, c)) = iter.next_if(|&(_, c)| c.is_ascii_hexdigit()) {
            uchr.push(c);
            end = i;
        }

        Ok((
            RegexAtom::Literal(
                // TODO: These unwrap()s are unbearable
                char::from_u32(u32::from_str_radix(uchr.as_str(), 16).unwrap()).unwrap(),
            ),
            Position { start, end },
        ))
    }

    fn lex_unicode_range<I>(&self, iter: &mut Peekable<I>) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        let mut uchr = String::new();

        let start;
        let mut end;

        let start_range: char;
        let end_range: char;

        match iter.next() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Unexpected end while parsing unicode range"
                )));
            }
            Some((i, c)) => {
                if c != '{' {
                    return Err(TsegerError::LexerError(format!(
                        "Unexpected character '{}' at position {}",
                        c, i
                    )));
                }

                start = i - 2;
            }
        }

        while let Some((i, c)) = iter.next() {
            end = i;

            match c {
                ',' => uchr.push(c),
                c if c.is_ascii_hexdigit() => uchr.push(c),
                '}' => {
                    if !uchr.contains(',') {
                        return Err(TsegerError::LexerError(format!(
                            "Invalid unicode range at {}",
                            i
                        )));
                    }

                    let split = uchr.split_once(',');

                    match split {
                        None => {
                            return Err(TsegerError::LexerError(format!(
                                "Invalid unicode range at {}",
                                i
                            )));
                        }
                        Some((s1, s2)) => {
                            // TODO: These unwrap() calls look ugly as hell
                            start_range =
                                char::from_u32(u32::from_str_radix(s1, 16).unwrap()).unwrap();
                            end_range =
                                char::from_u32(u32::from_str_radix(s2, 16).unwrap()).unwrap();

                            return Ok((
                                RegexAtom::Range(start_range, end_range),
                                Position { start, end },
                            ));
                        }
                    }
                }
                _ => {
                    return Err(TsegerError::LexerError(format!(
                        "Unexpected character '{}' at {}",
                        c, i
                    )));
                }
            }
        }

        // TODO: What the hell is even this
        Ok((RegexAtom::EOF, Position { start: 0, end: 0 }))
    }

    fn lex_variables<I>(&self, iter: &mut Peekable<I>) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        // TODO: This looks like shit, improve early return
        match iter.peek() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Invalid end of the expression, \\p is the last char"
                )));
            }
            Some((i, c)) => {
                if *c != '{' {
                    return Err(TsegerError::LexerError(format!(
                        "Invalid token '{}' after \\p at position: {}",
                        c, i
                    )));
                }
            }
        }

        let n = iter.next();

        let mut variable_name = String::new();
        let start;
        let mut end = 0;

        match n {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Invalid end of the expression \\p is the last char"
                )));
            }

            // TODO: This piece is screaming for refactoring
            Some((i, _)) => {
                start = i;
                while let Some(m) = iter.next() {
                    if m.1 == '}' {
                        end = m.0;
                        return Ok((RegexAtom::Variable(variable_name), Position { start, end }));
                    } else {
                        variable_name.push(m.1)
                    }
                }
            }
        }

        Ok((RegexAtom::Variable(variable_name), Position { start, end }))
    }

    fn lex_char_classes<I>(&self, iter: &mut Peekable<I>) -> Result<PositionedAtom, TsegerError>
    where
        I: Iterator<Item = (usize, char)>,
    {
        // TODO: This looks like shit, improve early return
        match iter.peek() {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Invalid end of the expression, \\p is the last char"
                )));
            }
            Some((i, c)) => {
                if *c != '{' {
                    return Err(TsegerError::LexerError(format!(
                        "Invalid token '{}' after \\p at position: {}",
                        c, i
                    )));
                }
            }
        }

        let n = iter.next();

        let mut class_name = String::new();
        let start;
        let mut end = 0;

        match n {
            None => {
                return Err(TsegerError::LexerError(format!(
                    "Invalid end of the expression \\p is the last char"
                )));
            }

            // TODO: This piece is screaming for refactoring
            Some((i, _)) => {
                start = i;
                while let Some(m) = iter.next() {
                    if m.1 == '}' {
                        end = m.0;
                        return Ok((RegexAtom::CharClass(class_name), Position { start, end }));
                    } else {
                        class_name.push(m.1)
                    }
                }
            }
        }

        Ok((RegexAtom::CharClass(class_name), Position { start, end }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_complex_test_1() {
        let rx = r#"ab{123,}|c*(abc)\n\\\?.?\p{digits}\p{pokemons}\xBEEF[aab-cD-04-999]\x{D3AD,BEEF}#{var1} #{var2}\x{1EE7,C0DE}
\p{testing}"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();
        let mut v = lexed.iter();

        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('a'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('b'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Repetition(Some(123), None));
        assert_eq!(v.next().unwrap().0, RegexAtom::Or);
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('c'));
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantKleene);
        assert_eq!(v.next().unwrap().0, RegexAtom::LParen);
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('a'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('b'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('c'));

        assert_eq!(v.next().unwrap().0, RegexAtom::RParen);
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::NewLine)
        );
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('\\'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('?'));
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantWildcard);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantOptional);
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::CharClass("digits".to_string())
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::CharClass("pokemons".to_string())
        );
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('\u{BEEF}'));
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::BracketExpressions {
                ranges: vec![
                    BracketExpression::Single((
                        RegexAtom::Literal('a'),
                        Position { start: 53, end: 53 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('a'),
                        Position { start: 54, end: 54 }
                    )),
                    BracketExpression::Ranged((
                        RegexAtom::Range('b', 'c'),
                        Position { start: 55, end: 57 }
                    )),
                    BracketExpression::Ranged((
                        RegexAtom::Range('D', '0'),
                        Position { start: 58, end: 60 }
                    )),
                    BracketExpression::Ranged((
                        RegexAtom::Range('4', '9'),
                        Position { start: 61, end: 63 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('9'),
                        Position { start: 64, end: 64 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('9'),
                        Position { start: 65, end: 65 }
                    )),
                ],
                negated: false
            }
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Range('\u{D3AD}', '\u{BEEF}'),
        );
        assert_eq!(v.next().unwrap().0, RegexAtom::Variable("var1".to_string()));
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Space)
        );
        assert_eq!(v.next().unwrap().0, RegexAtom::Variable("var2".to_string()));
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Range('\u{1EE7}', '\u{C0DE}')
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::NewLine),
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::CharClass("testing".to_string())
        );
    }

    #[test]
    fn ascii_ranges_test() {
        let rx = r#"[^aabba-zABC-Z01-9]"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();
        let mut v = lexed.iter();

        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::BracketExpressions {
                ranges: vec![
                    BracketExpression::Single((
                        RegexAtom::Literal('a'),
                        Position { start: 2, end: 2 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('a'),
                        Position { start: 3, end: 3 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('b'),
                        Position { start: 4, end: 4 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('b'),
                        Position { start: 5, end: 5 }
                    )),
                    BracketExpression::Ranged((
                        RegexAtom::Range('a', 'z'),
                        Position { start: 6, end: 8 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('A'),
                        Position { start: 9, end: 9 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('B'),
                        Position { start: 10, end: 10 }
                    )),
                    BracketExpression::Ranged((
                        RegexAtom::Range('C', 'Z'),
                        Position { start: 11, end: 13 }
                    )),
                    BracketExpression::Single((
                        RegexAtom::Literal('0'),
                        Position { start: 14, end: 14 }
                    )),
                    BracketExpression::Ranged((
                        RegexAtom::Range('1', '9'),
                        Position { start: 15, end: 17 }
                    )),
                ],
                negated: true
            }
        );
    }

    #[test]
    fn whitespaces_test() {
        let rx = r#"\t \n \r   \t
        "#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();
        let mut v = lexed.iter();

        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Tab)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Space)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::NewLine)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Space)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::CR)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Space)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Space)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Space)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Tab)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::NewLine)
        );
    }

    #[test]
    fn unicode_range_happy_path() {
        let rx = r#"\x{21A9,21B0}\x{21B2,21B9}"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();

        let mut v = lexed.iter();

        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Range('\u{21A9}', '\u{21B0}')
        );

        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Range('\u{21B2}', '\u{21B9}')
        );
    }

    #[test]
    fn unicode_char_happy_path() {
        let rx = r#"\x21A9\x21AA\x21AB"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();

        let mut v = lexed.iter();
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('\u{21A9}'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('\u{21AA}'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('\u{21AB}'));
    }

    #[test]
    fn variables_happy_path() {
        let rx = r#"#{var1}#{var2}"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();

        let mut v = lexed.iter();
        assert_eq!(v.next().unwrap().0, RegexAtom::Variable("var1".to_string()));
        assert_eq!(v.next().unwrap().0, RegexAtom::Variable("var2".to_string()));
    }

    #[test]
    fn char_classes_happy_path() {
        let rx = r#"\p{cls1}\p{cls2}"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();

        let mut v = lexed.iter();
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::CharClass("cls1".to_string())
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::CharClass("cls2".to_string())
        );
    }

    #[test]
    fn test_lex_one_symbol_quantifiers() {
        let rx = r#".*?+|"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();
        let mut v = lexed.iter();

        assert_eq!(v.next().unwrap().0, RegexAtom::QuantWildcard);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantKleene);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantOptional);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantPlus);
        assert_eq!(v.next().unwrap().0, RegexAtom::Or);
    }

    #[test]
    fn test_escape_characters() {
        let rx = r#"\(\)\[\]\{\}\.\*\?\+\^\|\\\#\n\r\t"#;
        let lexer = Lexer::new(rx);
        let lexed = lexer.lex().unwrap();
        let mut v = lexed.iter();

        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('('));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal(')'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('['));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal(']'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('{'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('}'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('.'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('*'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('?'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('+'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('^'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('|'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('\\'));
        assert_eq!(v.next().unwrap().0, RegexAtom::Literal('#'));
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::NewLine)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::CR)
        );
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::Whitespace(WhitespaceKind::Tab)
        );
    }
}
