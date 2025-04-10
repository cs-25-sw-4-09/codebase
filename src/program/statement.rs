use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;

use crate::program::r#type::Type;
use super::expression::Expr;

#[derive(Debug)]
pub enum Stmt {
    VarDecl {
        name: String,
        declared_type: Type,
        value: Expr,
    },
}

impl Stmt {
    pub fn new(stmt: AstNode) -> Self {
        match stmt.get_symbol().name {
            "varDecl" => {
                //let identifier = stmt.children().iter().find(|child| child.get_symbol().name.eq("IDENTIFIER")).unwrap().get_value().unwrap().into();
                Stmt::VarDecl {
                    name: stmt.child(0).get_value().unwrap().into(),
                    declared_type: Type::new(stmt.child(1).get_value().unwrap()),
                    value: Expr::new(stmt.child(2)),
                }
            }
            _ => panic!(),
        }
    }
}
