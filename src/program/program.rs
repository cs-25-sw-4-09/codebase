use hime_redist::{
    ast::AstNode,
    symbols::SemanticElementTrait,
};

use crate::{lexer_parser::grammar::cfg, typechecker::environment::TEnvironment};

use super::statement::Stmt;

#[derive(Debug)]
pub struct Program {
    pub decl_f: Vec<Stmt>,
    pub stmts: Vec<Stmt>,
    pub tenvironment: TEnvironment,
}

impl Program {
    pub fn new(input: &str) -> Self {
        let mut decl_f: Vec<Stmt> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        let tenvironment = TEnvironment::new();

        let parsed = cfg::parse_str(input);
        let ast = parsed.get_ast();
        let root_node = ast.get_root();

        for node in root_node.children() {
            match node.get_symbol().name {
                "declS" => {
                    for decl in node.children() {
                        decl_f.push(Stmt::new(decl));
                    }
                }
                "stmtS" => {
                    for stmt in node.children() {
                        stmts.push(Stmt::new(stmt));
                    }
                }
                _ => panic!(),
            }
        }

        Program {
            decl_f,
            stmts,
            tenvironment,
        }
    }
}

