use hime_redist::ast::AstNode;
use hime_redist::symbols::SemanticElementTrait;

use super::environment::TEnvironment;
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

    pub fn type_check(&self, environment: &mut TEnvironment) -> Result<Type, ()> {
        match self {
            Stmt::VarDecl {
                name,
                declared_type,
                value,
            } => {
                if environment.vtable_contains(name) {
                    return Err(());
                };
                if declared_type.eq(&value.type_check(environment)?) {
                    environment.vtable_set(name.clone(), declared_type.clone()); //måske fiks clone here
                    return Ok(declared_type.clone());
                }
                Err(())
            }
        }
    }
}
