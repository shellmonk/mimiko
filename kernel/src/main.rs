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

    match parser.parse(&mut lex) {
        Ok(prog) => println!("{prog:?}"),
        Err(e) => eprintln!("{}", e.to_string()),
    }
    //program.eval();

    Ok(())
}
