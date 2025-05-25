#[derive(Debug)]
pub struct ProgramAST {
    pub statements: Vec<StmtDecl>,
}

#[derive(Debug)]
pub enum StmtDecl {
    IncludeStmtDecl(IncludeStmt),
    LoadStmtDecl(LoadStmt),
    IngestStmtDecl(IngestStmt),
    GenStmtDecl(GenStmt),
    TypeStmtDecl(TypeStmt),
    DumpStmtDecl(DumpStmt),
    StaticStmtDecl(StaticStmt),
}

#[derive(Debug)]
pub struct LoadStmt {
    pub module: ModuleDef,
}

#[derive(Debug)]
pub struct IncludeStmt {
    pub module: ModuleDef,
}

#[derive(Debug)]
pub struct ModuleDef {
    pub namespaces: Vec<String>,
    pub alias: Option<String>,
}

#[derive(Debug)]
pub struct IngestStmt {}
#[derive(Debug)]
pub struct GenStmt {}
#[derive(Debug)]
pub struct TypeStmt {}
#[derive(Debug)]
pub struct DumpStmt {}
#[derive(Debug)]
pub struct StaticStmt {}
