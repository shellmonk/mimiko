use super::ast::*;
use super::common::MimikoError;
use super::lexer::Token;
use logos::Lexer;

#[derive(Debug)]
pub struct Parser {}

type ParserResult<T> = Result<T, MimikoError>;

impl Parser {
    pub fn parse(&self, lexer: &mut Lexer<Token>) -> ParserResult<ProgramAST> {
        //while let Some(tok) = lexer.next() {
        //    println!("TOK: {tok:?}");
        //}

        let mut statements: Vec<StmtDecl> = Vec::new();
        while let Some(Ok(token)) = lexer.next() {
            match token {
                Token::Load => statements.push(self.parse_load_statement(lexer)?),
                Token::Include => statements.push(self.parse_include_statement(lexer)?),
                Token::Ingest => statements.push(self.parse_ingest_statement(lexer)?),
                Token::Gen => statements.push(self.parse_generator_statement(lexer)?),
                Token::TypeObj => statements.push(self.parse_type_statement(lexer)?),
                Token::Dump => statements.push(self.parse_dump_statement(lexer)?),
                Token::Static => statements.push(self.parse_static_statement(lexer)?),
                _ => {
                    return Err(MimikoError::ParserUnexpectedToken {
                        token: lexer.slice().to_owned(),
                        range: lexer.span(),
                    });
                }
            }
        }

        Ok(ProgramAST { statements })
    }

    fn skip_statement(&self, lexer: &mut Lexer<Token>) {
        while let Some(Ok(tok)) = lexer.next() {
            if tok == Token::EndStmt {
                break;
            }
        }
    }

    fn parse_static_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        while let Some(Ok(tok)) = lexer.next() {
            if tok == Token::EndStmt {
                break;
            }
        }

        Ok(StmtDecl::StaticStmtDecl(StaticStmt {}))
    }

    fn parse_include_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        let mut namespaces: Vec<String> = Vec::new();
        let mut alias: Option<String> = None;

        let mut piter = lexer.peekable();
        let mut parsing_alias = false;

        while let Some(Ok(tok)) = piter.next() {
            if tok == Token::EndStmt && !namespaces.is_empty() {
                return Ok(StmtDecl::IncludeStmtDecl(IncludeStmt {
                    module: ModuleDef { namespaces, alias },
                }));
            }

            match tok {
                Token::As => parsing_alias = true,
                Token::Identifier(id) => {
                    if parsing_alias {
                        alias = Some(id)
                    } else {
                        namespaces.push(id)
                    }
                }
                Token::ScopeResolutionOp => {
                    if namespaces.is_empty() {
                        return Err(MimikoError::ParserUnexpectedToken {
                            range: lexer.span(),
                            token: lexer.slice().to_owned(),
                        });
                    }

                    match piter.peek() {
                        None => {
                            return Err(MimikoError::ParserUnexpectedEndSequence {
                                range: lexer.span(),
                            });
                        }
                        Some(Ok(t)) => {
                            if !matches!(Token::Identifier, t) {
                                return Err(MimikoError::ParserUnexpectedEndSequence {
                                    range: lexer.span(),
                                });
                            }
                        }
                        _ => todo!("This case is weird, investigate and do proper error handling"),
                    }
                }
                _ => {
                    return Err(MimikoError::ParserUnexpectedToken {
                        range: lexer.span(),
                        token: lexer.slice().to_owned(),
                    });
                }
            }
        }

        Ok(StmtDecl::IncludeStmtDecl(IncludeStmt {
            module: ModuleDef { namespaces, alias },
        }))
    }

    fn parse_load_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        let mut namespaces: Vec<String> = Vec::new();
        let mut alias: Option<String> = None;

        let mut piter = lexer.peekable();
        let mut parsing_alias = false;

        while let Some(Ok(tok)) = piter.next() {
            if tok == Token::EndStmt && !namespaces.is_empty() {
                return Ok(StmtDecl::LoadStmtDecl(LoadStmt {
                    module: ModuleDef { namespaces, alias },
                }));
            }

            match tok {
                Token::As => parsing_alias = true,
                Token::Identifier(id) => {
                    if parsing_alias {
                        alias = Some(id)
                    } else {
                        namespaces.push(id)
                    }
                }
                Token::ScopeResolutionOp => {
                    if namespaces.is_empty() {
                        return Err(MimikoError::ParserUnexpectedToken {
                            range: lexer.span(),
                            token: lexer.slice().to_owned(),
                        });
                    }

                    match piter.peek() {
                        None => {
                            return Err(MimikoError::ParserUnexpectedEndSequence {
                                range: lexer.span(),
                            });
                        }
                        Some(Ok(t)) => {
                            if !matches!(Token::Identifier, t) {
                                return Err(MimikoError::ParserUnexpectedEndSequence {
                                    range: lexer.span(),
                                });
                            }
                        }
                        _ => todo!("This case is weird, investigate and do proper error handling"),
                    }
                }
                _ => {
                    return Err(MimikoError::ParserUnexpectedToken {
                        range: lexer.span(),
                        token: lexer.slice().to_owned(),
                    });
                }
            }
        }

        Ok(StmtDecl::LoadStmtDecl(LoadStmt {
            module: ModuleDef { namespaces, alias },
        }))
    }

    fn parse_ingest_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        while let Some(Ok(tok)) = lexer.next() {
            if tok == Token::EndStmt {
                break;
            }
        }
        Ok(StmtDecl::IngestStmtDecl(IngestStmt {}))
    }

    fn parse_generator_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        while let Some(Ok(tok)) = lexer.next() {
            if tok == Token::EndStmt {
                break;
            }
        }

        Ok(StmtDecl::GenStmtDecl(GenStmt {}))
    }

    fn parse_type_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        while let Some(Ok(tok)) = lexer.next() {
            if tok == Token::Var || tok == Token::Global {
                self.skip_statement(lexer);
            }
            if tok == Token::EndStmt {
                break;
            }
        }

        Ok(StmtDecl::TypeStmtDecl(TypeStmt {}))
    }

    fn parse_dump_statement(&self, lexer: &mut Lexer<Token>) -> ParserResult<StmtDecl> {
        while let Some(Ok(tok)) = lexer.next() {
            if tok == Token::EndStmt {
                break;
            }
        }

        Ok(StmtDecl::DumpStmtDecl(DumpStmt {}))
    }
}
