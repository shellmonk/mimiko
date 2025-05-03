#[derive(Debug, Clone)]
pub enum RegexToken {
    Literal(char), // regular character
    Number(i64),   // 1, 7, 69, 420
    LParen,        // (
    RParen,        // )
    LBrace,        // {
    RBrace,        // }
    LBracket,      // [
    RBracket,      // ]
    OpOr,          // |
    OpStar,        // *
    OpPlus,        // +
    OpQMark,       // ?
    OpWildcard,    // .
    OpEscapeChar,  // \
    OpSh,          // $
    Range,         // -
    Comma,         // ,
    Var(String),   // ${variable}
    Whitespace,    //
    NewLine,
    Tab,
    EOF,
}

pub fn lex(rx: &str) -> Vec<RegexToken> {
    let mut iter = rx.chars().into_iter().peekable();
    let mut tokens = Vec::new();

    loop {
        let c = iter.next();
        match c {
            Some('(') => tokens.push(RegexToken::LParen),
            Some(')') => tokens.push(RegexToken::RParen),
            Some('{') => {
                tokens.push(RegexToken::LBrace);
                let mut tmp_num = String::new();

                if iter.peek().unwrap().is_ascii_digit() {
                    while let Some(n) = iter.peek() {
                        if n.is_ascii_digit() {
                            tmp_num.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                }

                if !tmp_num.is_empty() {
                    tokens.push(RegexToken::Number(tmp_num.parse::<i64>().unwrap()));
                    tmp_num.clear();
                }
            }
            Some('}') => tokens.push(RegexToken::RBrace),
            Some('|') => tokens.push(RegexToken::OpOr),
            Some('*') => tokens.push(RegexToken::OpStar),
            Some('+') => tokens.push(RegexToken::OpPlus),
            Some('?') => tokens.push(RegexToken::OpQMark),
            Some('.') => tokens.push(RegexToken::OpWildcard),
            Some('\\') => tokens.push(RegexToken::OpEscapeChar),
            Some('$') => tokens.push(RegexToken::OpSh),
            Some('-') => tokens.push(RegexToken::Range),
            Some('\t') => tokens.push(RegexToken::Tab),
            Some('\n') => tokens.push(RegexToken::NewLine),
            Some(' ') => tokens.push(RegexToken::Whitespace),
            Some(',') => {
                tokens.push(RegexToken::Comma);

                let mut tmp_num = String::new();

                if iter.peek().unwrap().is_ascii_digit() {
                    while let Some(n) = iter.peek() {
                        if n.is_ascii_digit() {
                            tmp_num.push(iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                }

                if !tmp_num.is_empty() {
                    tokens.push(RegexToken::Number(tmp_num.parse::<i64>().unwrap()));
                    tmp_num.clear();
                }
            }
            None => {
                tokens.push(RegexToken::EOF);
                break;
            }
            c if c.unwrap().is_alphanumeric() => tokens.push(RegexToken::Literal(c.unwrap())),
            _ => panic!("Unexpected character '{}'", c.unwrap()),
        }
    }

    println!("{:?}", tokens);

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lex() {
        let v = lex("a|(b.x?){5,20}?asdf.,.,,,...12315552138");
        println!("{:?}", v);
    }
}
