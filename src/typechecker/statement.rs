use std::{collections::HashMap, error::Error};

use crate::{
    program::{program::Program, r#type::Type, statement::Stmt},
    typechecker::TypeCheckP,
};

use super::{environment::TEnvironment, TypeCheckE, TypeCheckS};

impl TypeCheckS for Stmt {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<(), Box<dyn Error>> {
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
                    environment.vtable_set(name.clone(), declared_type.clone());
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
                    let (_, parameter_types): (Vec<_>, Vec<Type>) =
                        parameters.iter().cloned().unzip();
                    environment.ftable_set(name.clone(), parameter_types, return_type.clone());
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
            Stmt::Decl {
                name,
                declared_type,
                value,
            } => {
                if environment.vdtable_contains(name) {
                    return Err(());
                };
                if let Some(set_value) = value {
                    if declared_type.eq(&set_value.type_check(environment)?) {
                        environment.vdtable_set(name.clone(), declared_type.clone());
                        Ok(())
                    } else {
                        Err(())
                    }
                } else {
                    environment.vdtable_set(name.clone(), declared_type.clone());
                    Ok(())
                }
            }
            Stmt::Import { name, path } => {
                if environment.stable_contains(name) {
                    return Err(());
                }

                let mut subprogram = Program::new(path)?;

                match subprogram.type_check() {
                    Ok(subprogram_environment) => {
                        println!("[Typechecker] Path: {} - OK", path);
                        let parameters: HashMap<String, Type> =
                            subprogram_environment.vdtable_get_hashmap();

                        environment.stable_set(name.clone(), parameters);
                        Ok(())
                    }
                    Err(_) => {
                        println!("[Typechecker] Path: {} - ERROR", path);
                        Err(())
                    }
                }
            }
        }
    }
}
