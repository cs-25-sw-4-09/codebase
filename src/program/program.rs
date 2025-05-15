use std::{error::Error, fs, path::Path};
use hime_redist::symbols::SemanticElementTrait;
use crate::{interpreter::environment::IEnvironment, lexer_parser::grammar::cfg, typechecker::environment::TEnvironment};
use super::{errors, statement::Stmt};

#[derive(Debug, Clone)]
pub struct Program {
    pub decl_f: Vec<Stmt>,
    pub stmts: Vec<Stmt>,
    pub tenvironment: TEnvironment,
    pub ienvironment: IEnvironment,
}

impl Program {
    pub fn new(programstr: &String) -> Result<Self, Box<dyn Error>> {
        let mut decl_f: Vec<Stmt> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        let tenvironment = TEnvironment::new();
        let ienvironment = IEnvironment::new();

        let parsed = cfg::parse_string(programstr.into());
        if !parsed.is_success() {
            return Err(errors::HimeParseMalfunction.into());
        }
        let ast = parsed.get_ast();
        let root_node = ast.get_root();

        for node in root_node.children() {
            match node.get_symbol().name {
                "declS" => {
                    for decl in node.children() {
                        decl_f.push(Stmt::new(decl)?);
                    }
                }
                "stmtS" => {
                    for stmt in node.children() {
                        stmts.push(Stmt::new(stmt)?);
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(Program {
            decl_f,
            stmts,
            tenvironment,
            ienvironment
        })
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let programstr = fs::read_to_string(path)?;

        Self::new(&programstr)
    }
}