use super::common::MimikoError;
use super::lexer::Token;
use logos::Lexer;

pub struct Parser {}

type ParserResult<T> = Result<T, MimikoError>;

impl Parser {
    pub fn parse(&self, lexer: &mut Lexer<Token>) -> ParserResult<ProgramAST> {
        while let Some(t) = lexer.next() {
            println!("LEXER NEXT: {:?}", t);
        }

        self.parse_program(lexer)
    }

    fn parse_program(&self, lexer: &mut Lexer<Token>) -> ParserResult<ProgramAST> {
        let mut statements = Vec::new();

        Ok(ProgramAST::new(statements))
    }

    fn parse_statement_decl(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        Ok(StmtDecl::UseStmtDecl(UseStmt {
            modules: Vec::new(),
        }))
    }
}

pub struct ProgramAST {
    statements: Vec<StmtDecl>,
}

impl ProgramAST {
    fn new(statements: Vec<StmtDecl>) -> Self {
        Self { statements }
    }
}

pub enum StmtDecl {
    UseStmtDecl(UseStmt),
    IngestStmtDecl(IngestStmt),
    GenStmtDecl(GenStmt),
    TypeStmtDecl(TypeStmt),
    DumpStmtDecl(DumpStmt),
}

struct UseStmt {
    modules: Vec<ModuleDef>,
}

struct ModuleDef {
    name: String,
    alias: String,
}

struct IngestStmt {}
struct GenStmt {}
struct TypeStmt {}
struct DumpStmt {}
