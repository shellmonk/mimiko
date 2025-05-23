#[derive(Debug)]
pub struct ProgramAST {
    pub statements: Vec<StmtDecl>,
}

#[derive(Debug)]
pub enum StmtDecl {
    UseStmtDecl(UseStmt),
    IngestStmtDecl(IngestStmt),
    GenStmtDecl(GenStmt),
    TypeStmtDecl(TypeStmt),
    DumpStmtDecl(DumpStmt),
}

#[derive(Debug)]
pub struct UseStmt {
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
