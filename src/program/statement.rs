use std::error::Error;

use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;

use super::expression::Expr;
use crate::program::errors;
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
    Draw {
        shape: Expr,
        point: Option<Expr>,
    }
}

impl Stmt {
    pub fn new(stmt: AstNode) -> Result<Self, Box<dyn Error>> {
        let stmt = match stmt.get_symbol().name {
            "varDecl" => {
                if stmt.children_count() != 3 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(3, stmt.children_count()).into(),
                    );
                }
                Stmt::VarDecl {
                    name: stmt
                        .child(0)
                        .get_value()
                        .ok_or_else(|| {
                            errors::ASTNodeValueInvalid(stmt.child(0).get_symbol().name.to_owned())
                        })?
                        .into(),
                    declared_type: Type::new(stmt.child(1).get_value().ok_or_else(|| {
                        errors::ASTNodeValueInvalid(stmt.child(0).get_symbol().name.to_owned())
                    })?)?,
                    value: Expr::new(stmt.child(2))?,
                }
            }
            "funcDecl" => {
                if stmt.children_count() != 4 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(4, stmt.children_count()).into(),
                    );
                }
                let mut parameters = Vec::new();
                let mut statements = Vec::new();
                for param in stmt.child(1).children() {
                    if param.children_count() != 2 {
                        return Err(
                            errors::ASTNodeChildrenCountInvalid(2, stmt.children_count()).into(),
                        );
                    }
                    let parameter: (String, Type) = (
                        param
                            .child(0)
                            .get_value()
                            .ok_or_else(|| {
                                errors::ASTNodeValueInvalid(
                                    param.child(0).get_symbol().name.to_owned(),
                                )
                            })?
                            .into(),
                        Type::new(param.child(1).get_value().ok_or_else(|| {
                            errors::ASTNodeValueInvalid(param.child(0).get_symbol().name.to_owned())
                        })?)?,
                    );

                    if parameters.contains(&parameter) {
                        return Err(errors::ParemeterAlreadyDefined(parameter.0.to_owned()).into());
                    }

                    parameters.push(parameter);
                }

                for stmt in stmt.child(3).children() {
                    statements.push(Stmt::new(stmt)?);
                }

                Stmt::FuncDecl {
                    name: stmt
                        .child(0)
                        .get_value()
                        .ok_or_else(|| {
                            errors::ASTNodeValueInvalid(stmt.child(0).get_symbol().name.to_owned())
                        })?
                        .into(),
                    return_type: Type::new(stmt.child(2).get_value().ok_or_else(|| {
                        errors::ASTNodeValueInvalid(stmt.child(2).get_symbol().name.to_owned())
                    })?)?,
                    parameters,
                    statements,
                }
            }
            "decl" => {
                if !(stmt.children_count() == 3 || stmt.children_count() == 2) {
                    return Err(errors::ASTNodeChildrenCountInvalidEither(
                        2,
                        3,
                        stmt.children_count(),
                    )
                    .into());
                }
                Stmt::Decl {
                    name: stmt
                        .child(0)
                        .get_value()
                        .ok_or_else(|| {
                            errors::ASTNodeValueInvalid(stmt.child(0).get_symbol().name.to_owned())
                        })?
                        .into(),
                    declared_type: Type::new(stmt.child(1).get_value().ok_or_else(|| {
                        errors::ASTNodeValueInvalid(stmt.child(1).get_symbol().name.to_owned())
                    })?)?,
                    value: if stmt.children_count() > 2 {
                        Some(Expr::new(stmt.child(2))?)
                    } else {
                        None
                    },
                }
            }
            "return" => {
                if stmt.children_count() != 1 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(1, stmt.children_count()).into(),
                    );
                }
                Stmt::Return(Expr::new(stmt.child(0))?)
            }
            "import" => {
                if stmt.children_count() != 2 {
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(2, stmt.children_count()).into(),
                    );
                }
                Stmt::Import {
                    name: stmt
                        .child(0)
                        .get_value()
                        .ok_or_else(|| {
                            errors::ASTNodeValueInvalid(stmt.child(0).get_symbol().name.to_owned())
                        })?
                        .into(),
                    path: stmt
                        .child(1)
                        .get_value()
                        .ok_or_else(|| {
                            errors::ASTNodeValueInvalid(stmt.child(1).get_symbol().name.to_owned())
                        })?
                        .replace('"', ""),
                }
            },
            "draw" => {
                if stmt.children_count() == 1 {
                    Stmt::Draw { shape: Expr::new(stmt.child(0))?
                    , point: None }

                } else if stmt.children_count() == 2{
                    Stmt::Draw { shape: Expr::new(stmt.child(0))?
                        , point: Some(Expr::new(stmt.child(1))?) }
                } else{
                    return Err(
                        errors::ASTNodeChildrenCountInvalid(0, stmt.children_count()).into(),
                    );
                }
            }
            _ => unreachable!(),
        };

        Ok(stmt)
    }
}
