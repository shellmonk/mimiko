use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[ \t\n\r\f]+")]
#[logos(skip r"//.*")]
pub enum Token {
    #[regex(r#"[a-zA-Z][a-zA-Z0-9_-]*"#, mimiko_identifier, priority = 3)]
    Identifier(String),
    #[regex(r#""[^"]*""#, mimiko_string, priority = 1)]
    String(String),
    #[regex(r#"[0-9]+"#, mimiko_integer, priority = 1)]
    Integer(i64),
    #[regex(r#"[0-9]+(\.[0-9]+)"#, mimiko_float, priority = 1)]
    Float(f64),

    #[token("(")]
    LtParen,
    #[token(")")]
    RtParen,
    #[token("[")]
    LtBracket,
    #[token("]")]
    RtBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("!")]
    Exec,
    #[token("_")]
    Underscore,
    #[token(":")]
    TypeDef,
    #[token(";")]
    EndStmt,

    #[token("=")]
    Assignment,
    #[token("+")]
    Addition,
    #[token("-")]
    Subtraction,
    #[token("*")]
    Multiplication,
    #[token("/")]
    Division,

    #[token("->")]
    Return,
    #[token("|>")]
    PipeOp,
    #[token("::")]
    ScopeResolutionOp,

    #[token("str")]
    StringType,
    #[token("int")]
    IntType,
    #[token("float")]
    FloatType,
    #[token("bool")]
    BoolType,

    #[token("true")]
    BoolValTrue,
    #[token("false")]
    BoolValFalse,

    #[token("static")]
    Static,
    #[token("var")]
    Var,
    #[token("global")]
    Global,
    #[token("include")]
    Include,
    #[token("load")]
    Load,
    #[token("as")]
    As,
    #[token("ingest")]
    Ingest,
    #[token("gen")]
    Gen,
    #[token("regex")]
    Regex,
    #[token("asc")]
    Ascending,
    #[token("desc")]
    Descending,
    #[token("rand")]
    Random,
    #[token("type")]
    TypeObj,
    #[token("dump")]
    Dump,
    #[token("self")]
    SelfRef,
    #[token("ext")]
    ExternalRef,
}

fn mimiko_integer(lexer: &mut logos::Lexer<Token>) -> i64 {
    let slice = lexer.slice();
    slice.parse::<i64>().expect(
        format!(
            "Error parsing int {}:{}",
            lexer.span().start,
            lexer.span().end
        )
        .as_str(),
    )
}

fn mimiko_float(lexer: &mut logos::Lexer<Token>) -> f64 {
    let slice = lexer.slice();
    slice.parse::<f64>().expect(
        format!(
            "Error parsing float {}:{}",
            lexer.span().start,
            lexer.span().end
        )
        .as_str(),
    )
}

fn mimiko_string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_owned()
}

fn mimiko_identifier(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice.to_owned()
}
