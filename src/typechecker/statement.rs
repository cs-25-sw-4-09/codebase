use crate::program::statement::Stmt;

use super::{environment::TEnvironment, TypeCheckE, TypeCheckS};

impl TypeCheckS for Stmt {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<crate::program::r#type::Type, ()> {
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