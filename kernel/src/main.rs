mod ast;
mod cli;
mod common;
mod lexer;
mod parser;

use std::fs;

use logos::Logos;
use parser::Parser;

fn main() -> Result<(), ()> {
    let src = fs::read_to_string("docs/syntax-sketches/v1.mim").expect("shit happens");
    let mut lex = lexer::Token::lexer(src.as_str());
    let parser = Parser {};

    let program = parser.parse(&mut lex).map_err(|e| eprintln!("{e:?}"));

    println!("{program:?}");
    //program.eval();

    Ok(())
}
