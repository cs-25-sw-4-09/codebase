use hime_redist::{
    ast::AstNode,
    symbols::{SemanticElementTrait, Symbol},
};

use super::{environment::TEnvironment, statement::Stmt};

#[derive(Debug)]
pub struct Program {
    pub decl_f: Vec<Decl>,
    pub stmts: Vec<Stmt>,
    pub tenvironment: TEnvironment,
}

impl Program {
    pub fn new(root_node: AstNode) -> Self {
        let mut decl_f: Vec<Decl> = Vec::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        let tenvironment = TEnvironment::new();

        for node in root_node.children() {
            match node.get_symbol().name {
                "declS" => {
                    for decl in node.children() {
                        todo!()
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

    pub fn type_check(&mut self) {
        self.stmts.iter().for_each(|stmt| {
            println!(
                "{}",
                if stmt.type_check(&mut self.tenvironment).is_ok() {
                    "TRUE"
                } else {
                    "FALSE"
                }
            )
        })
    }
}

#[derive(Debug)]
pub enum Decl {
    Import {},
    Decl {},
}

