use std::error::Error;

use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;

use super::expression::Expr;
use crate::program::r#type::Type;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    VarDecl {
        name: String,
        declared_type: Type,
        value: Expr,
    },
    FuncDecl {
        name: String,
        return_type: Type,
        parameters: Vec<(String, Type)>,
        statements: Vec<Stmt>,
    },
    Return(Expr),
    Decl {
        name: String,
        declared_type: Type,
        value: Option<Expr>,
    },
    Import {
        name: String,
        path: String,
    },
}

impl Stmt {
    pub fn new(stmt: AstNode) -> Result<Self, Box<dyn Error>> {
        let stmt = match stmt.get_symbol().name {
            "varDecl" => {
                //let identifier = stmt.children().iter().find(|child| child.get_symbol().name.eq("IDENTIFIER")).unwrap().get_value().unwrap().into();
                Stmt::VarDecl {
                    name: stmt.child(0).get_value().unwrap().into(),
                    declared_type: Type::new(stmt.child(1).get_value().unwrap())?,
                    value: Expr::new(stmt.child(2))?,
                }
            }
            "funcDecl" => {
                let mut parameters = Vec::new();
                let mut statements = Vec::new();
                for param in stmt.child(1).children() {
                    let parameter = (
                        param.child(0).get_value().unwrap().into(),
                        Type::new(param.child(1).get_value().unwrap())?,
                    );

                    if parameters.contains(&parameter) {
                        panic!();
                    }

                    parameters.push(parameter);
                }

                for stmt in stmt.child(3).children() {
                    statements.push(Stmt::new(stmt)?);
                }

                Stmt::FuncDecl {
                    name: stmt.child(0).get_value().unwrap().into(),
                    return_type: Type::new(stmt.child(2).get_value().unwrap())?,
                    parameters,
                    statements,
                }
            }
            "decl" => Stmt::Decl {
                name: stmt.child(0).get_value().unwrap().into(),
                declared_type: Type::new(stmt.child(1).get_value().unwrap())?,
                value: if stmt.children_count() > 2 {
                    Some(Expr::new(stmt.child(2))?)
                } else {
                    None
                },
            },
            "return" => Stmt::Return(Expr::new(stmt.child(0))?),
            "import" => Stmt::Import {
                name: stmt.child(0).get_value().unwrap().into(),
                path: stmt.child(1).get_value().unwrap().replace('"', ""),
            },
            stmt => panic!("{}", stmt),
        };

        Ok(stmt)
    }
}
