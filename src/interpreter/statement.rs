use crate::program::{expression::Expr, statement::Stmt};

use super::{errors, InterpretE, InterpretS, value::Value};

impl InterpretS for Stmt {
    fn interpret(
        &self,
        environment: &mut super::environment::IEnvironment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if environment.rvalue_get().is_some() {
            return Ok(());
        }
        match self {
            Stmt::VarDecl {
                name,
                declared_type: _,
                value,
            } => {
                let i1 = value.interpret(environment)?;

                environment.vtable_push(name.into(), i1)
            }
            Stmt::FuncDecl {
                name,
                return_type: _,
                parameters,
                statements,
            } => {
                environment.ftable_push(
                    name.into(),
                    statements.clone(),
                    parameters.iter().map(|p| p.0.clone()).collect(),
                );
            }
            Stmt::Return(expr) => {
                let i1 = expr.interpret(environment)?;
                environment.rvalue_set(i1);
            }
            Stmt::Decl {
                name,
                declared_type:_,
                value,
            } => {
                if let Some(value) = value {
                    match environment.vtable_find(name.to_owned()) {
                        Some(_) => (),
                        None => {
                            let i1 = value.interpret(environment)?;
                            environment.vtable_push(name.to_owned(), i1);
                        }
                    }
                } else {
                    match environment.vtable_find(name.to_owned()) {
                        Some(_) => (),
                        None => return Err(errors::DeclValueNotSpecified(name.to_owned()).into()),
                    }
                }
            }
            Stmt::Import { name, path } => todo!(),
            Stmt::Draw { shape, point } => todo!(),
            Stmt::Assign { name, value } => {
                *environment.vtable_find(name.into()).unwrap() = value.interpret(environment)?;
            }
            Stmt::For {
                counter,
                from,
                to,
                body,
            } => {
                for i in
                    from.interpret(environment)?.get_int()?..to.interpret(environment)?.get_int()?
                {
                    environment.push_scope();
                    environment.vtable_push(counter.into(), Value::Integer(i));
                    for stmt in body.iter() {
                        stmt.interpret(environment)?;
                    }

                    environment.pop_scope();
                    if environment.rvalue_get().is_some() {
                        break;
                    }
                }
            }
            Stmt::Fork { branches, otherwise } => {
                for (expr, stmts) in branches {
                    if expr.interpret(environment)? == Value::Boolean(true) {
                        environment.push_scope();
                        for stmt in stmts {
                            stmt.interpret(environment)?;
                        }
                        environment.pop_scope();
                        return Ok(());
                    }
                }

                if let Some(otherwise) = otherwise {
                    environment.push_scope();
                    for stmt in otherwise {
                        stmt.interpret(environment)?;
                    }
                    environment.pop_scope();
                }
            },
        }

        Ok(())
    }
}
