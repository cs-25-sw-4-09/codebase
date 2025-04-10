use crate::program::statement::Stmt;

use super::{environment::TEnvironment, TypeCheckE, TypeCheckS};

impl TypeCheckS for Stmt {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<(), ()> {
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
                    return Ok(());
                }
                Err(())
            }
            Stmt::FuncDecl {
                name,
                return_type,
                parameters,
                statements,
            } => {
                if environment.ftable_contains(name) {
                    return Err(());
                } else {
                    environment.ftable_set(name.clone(), parameters.clone(), return_type.clone());
                }
                let mut new_environment = environment.clone();
                new_environment.r_type = Some(return_type.clone());

                for (param_name, param_type) in parameters {
                    new_environment.vtable_set(param_name.clone(), param_type.clone());
                }


                for stmt in statements {
                    stmt.type_check(&mut new_environment)?;
                }

                Ok(())
            }
            Stmt::Return(expr) => {
                let t1 = expr.type_check(environment)?;
                if environment.r_type.eq(&Some(t1)) {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }
}
