mod cli;
mod lexer;
mod parser;

use std::fs;

use logos::Logos;

fn main() {
    let src = fs::read_to_string("../docs/syntax-sketches/v1.mim").expect("shit happens");
    let mut lex = lexer::Token::lexer(src.as_str());

    while let Some(t) = lex.next() {
        println!("LEX: {:?}", t);
    }
}
