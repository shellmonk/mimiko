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
* -
*/
#[derive(Debug, PartialEq, Eq)]
pub enum RegexAtom {
    Literal(char),                        // ascii - abc123; unicode scalar values - \xFFFF
    Range(char, char),                    // ascii - [a-zA-Z0-9]; unicode ranges - \x{FFF0,FFFF}
    QuantWildcard,                        // . {1,1}
    QuantOptional,                        // ? {0,1}
    QuantKleene,                          // * {0,}
    QuantPlus,                            // + {1,}
    Or,                                   // | logical OR
    LParen,                               // (
    RParen,                               // )
    Repetition(Option<u32>, Option<u32>), // {69,420}
    Whitespace(WhitespaceKind),           // \t \r \n
    Negation,                             // ^
    CharClass(String),                    // \p{digits}
    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WhitespaceKind {
    NewLine,
    Tab,
    CR,
}

#[derive(Debug)]
pub struct Position {
    start: usize,
    end: usize,
}

pub struct Lexer {}

pub fn lex(rx: &str) -> Vec<(RegexAtom, Position)> {
    let mut tokens = Vec::new();

    let mut iter = rx.chars().enumerate().into_iter().peekable();

    let mut in_range = false;
    let mut in_char_class = false;
    let mut in_repetition = false;

    while let Some((i, c)) = iter.next() {
        match c {
            '(' => tokens.push((RegexAtom::LParen, Position { start: i, end: i })),
            ')' => tokens.push((RegexAtom::RParen, Position { start: i, end: i })),
            '.' => tokens.push((RegexAtom::QuantWildcard, Position { start: i, end: i })),
            '*' => tokens.push((RegexAtom::QuantKleene, Position { start: i, end: i })),
            '?' => tokens.push((RegexAtom::QuantOptional, Position { start: i, end: i })),
            '+' => tokens.push((RegexAtom::QuantPlus, Position { start: i, end: i })),
            '^' => tokens.push((RegexAtom::Negation, Position { start: i, end: i })),
            '|' => tokens.push((RegexAtom::Or, Position { start: i, end: i })),
            '\\' => match iter.next() {
                None => todo!("This needs to be handled, unescaped slash at the end of the regex"),
                Some(following) => {
                    let fc = following.1;
                    match fc {
                        fc if vec![
                            '(', ')', '[', ']', '{', '}', '.', '*', '?', '+', '^', '|', '\\',
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
                        'x' => {
                            todo!("Unicode characters and ranges not yet implemented")
                        }
                        'p' => tokens.push(lex_char_classes(&mut iter)),
                        _ => todo!("This needs to be handled, unknown character after slash"),
                    }
                }
            },
            _ => todo!("Case {} at position {} not covered yet", c, i),
        }
    }

    tokens
}

fn lex_char_classes<I>(iter: &mut Peekable<I>) -> (RegexAtom, Position)
where
    I: Iterator<Item = (usize, char)>,
{
    // TODO: This looks like shit, improve early return
    match iter.peek() {
        None => todo!("Handle error when \\p is the last char"),
        Some((i, c)) => {
            if *c != '{' {
                todo!("Invalid token '{}' after \\p at position: {}", c, i);
            }
        }
    }

    let mut n = iter.next();

    let mut class_name = String::new();
    let mut start = 0;
    let mut end = 0;

    match n {
        None => todo!("String ending error handling after \\p should not yet implemented"),
        Some((i, _)) => {
            start = i;
            while let Some(m) = iter.next() {
                if m.1 == '}' {
                    end = m.0;
                    return (
                        RegexAtom::CharClass(class_name),
                        Position {
                            start: start,
                            end: end,
                        },
                    );
                } else {
                    class_name.push(m.1)
                }
            }
        }
    }

    (
        RegexAtom::CharClass(class_name),
        Position {
            start: start,
            end: end,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_classes_happy_path() {
        let rx = r#"\p{cls1}\p{cls2}"#;

        let lexed = lex(rx);
        println!("{:?}, {:?}", rx, lexed);

        let mut v = lexed.iter();
        assert_eq!(
            v.next().unwrap().0,
            RegexAtom::CharClass("cls1".to_string())
        );
    }

    #[test]
    fn test_lex_one_symbol_quantifiers() {
        let lexed = lex(".*?+^|");
        let mut v = lexed.iter();

        assert_eq!(v.next().unwrap().0, RegexAtom::QuantWildcard);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantKleene);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantOptional);
        assert_eq!(v.next().unwrap().0, RegexAtom::QuantPlus);
        assert_eq!(v.next().unwrap().0, RegexAtom::Negation);
        assert_eq!(v.next().unwrap().0, RegexAtom::Or);
    }

    #[test]
    fn test_escape_characters() {
        let rx = r#"\(\)\[\]\{\}\.\*\?\+\^\|\\\n\r\t"#;
        let lexed = lex(rx);
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
