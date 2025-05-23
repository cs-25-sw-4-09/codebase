use std::{collections::HashMap, error::Error, path::Path};

use crate::{
    program::{expression::Expr, program::Program, r#type::Type, statement::Stmt},
    typechecker::{environment::EType, TypeCheckP},
};

use super::{environment::TEnvironment, errors, TypeCheckE, TypeCheckS};

impl TypeCheckS for Stmt {
    fn type_check(&self, environment: &mut TEnvironment) -> Result<(), Box<dyn Error>> {
        match self {
            Stmt::Import { name, path } => {
                if environment.stable_lookup(name).is_ok() {
                    return Err(errors::ImportAlreadyDeclared(name.to_owned()).into());
                }

                let mut subprogram =
                    Program::from_file(Path::new(path)).map_err(|err| format!("{}", err))?;

                match subprogram.type_check() {
                    Ok(subprogram_environment) => {
                        println!("[Typechecker] Path: {} - OK", path);
                        let parameters: HashMap<String, EType> =
                            subprogram_environment.vdtable_get_hashmap();

                        environment.stable_set(name.clone(), parameters);
                        Ok(())
                    }
                    Err(err) => {
                        println!("[Typechecker] Path: {} - ERROR", path);
                        Err(err)
                    }
                }
            }
            Stmt::Decl {
                name,
                declared_type,
                value,
            } => {
                if environment.vdtable_lookup(name).is_ok() {
                    return Err(errors::IdentifierAlreadyDeclared(name.to_owned()).into());
                };
                if let Some(set_value) = value {
                    let t1 = set_value.type_check(environment)?;
                    if declared_type.eq(&t1) {
                        environment.vdtable_set_default(name.clone(), *declared_type);
                        Ok(())
                    } else if checks_empty_array(*declared_type, t1) {
                        environment.vdtable_set_default(name.clone(), declared_type.clone());
                        return Ok(());
                    } else {
                        Err(errors::VariableExpressionTypeNotMatch(
                            name.to_owned(),
                            declared_type.to_owned(),
                            t1,
                        )
                        .into())
                    }
                } else {
                    environment.vdtable_set_non_default(name.clone(), *declared_type);
                    Ok(())
                }
            }
            Stmt::VarDecl {
                name,
                declared_type,
                value,
            } => {
                if environment.vtable_lookup(name).is_ok() {
                    return Err(errors::IdentifierAlreadyDeclared(name.to_owned()).into());
                };
                let t1 = value.type_check(environment)?;
                if declared_type.eq(&t1) {
                    environment.vtable_set(name.clone(), *declared_type);
                    return Ok(());
                } else if checks_empty_array(*declared_type, t1) {
                    environment.vtable_set(name.clone(), *declared_type);
                    return Ok(());
                }
                Err(
                    errors::VariableExpressionTypeNotMatch(name.to_owned(), *declared_type, t1)
                        .into(),
                )
            }
            Stmt::Assign { name, value } => {
                let t1 = Expr::Variable(name.into()).type_check(environment)?;
                let t2 = value.type_check(environment)?;

                if t1 == t2 {
                    Ok(())
                } else if checks_empty_array(t1, t2) {
                    Ok(())
                } else {
                    Err(errors::AssignTypesNoMatch(t1, t2).into())
                }
            }
            Stmt::ArrayAssign { name, value, index } => {
                let t1 = Expr::Variable(name.into()).type_check(environment)?;
                let t2 = value.type_check(environment)?;
                let t3 = index.type_check(environment)?;

                if t3 != Type::Int {
                    return Err(errors::ArrayIndexTypeError(t3).into());
                }

                let array_type;
                match t1 {
                    Type::IntArray => array_type = Type::Int,
                    Type::BoolArray => array_type = Type::Bool,
                    Type::FloatArray => array_type = Type::Float,
                    Type::ShapeArray => array_type = Type::Shape,
                    Type::PointArray => array_type = Type::Point,
                    Type::ColorArray => array_type = Type::Color,
                    Type::PathArray => array_type = Type::Path,
                    Type::PolygonArray => array_type = Type::Polygon,
                    _ => unreachable!(), //return Err(errors::AssignTypesNoMatch(t1, t2).into())
                }

                if array_type == t2 {
                    Ok(())
                } else {
                    Err(errors::ArrayAssignTypeConflict(t2, array_type).into())
                }
            }

            Stmt::FuncDecl {
                name,
                return_type,
                parameters,
                statements,
            } => {
                if environment.ftable_lookup(name).is_ok() {
                    return Err(errors::IdentifierAlreadyDeclared(name.to_owned()).into());
                } else {
                    let (_, parameter_types): (Vec<_>, Vec<Type>) =
                        parameters.iter().cloned().unzip();
                    environment.ftable_set(name.clone(), parameter_types, *return_type);
                }
                let mut new_environment = environment.clone_and_clear_vtable();
                new_environment.return_set(*return_type);

                for (param_name, param_type) in parameters {
                    new_environment.vtable_set(param_name.clone(), *param_type);
                }

                for stmt in statements {
                    stmt.type_check(&mut new_environment)?;
                }

                Ok(())
            }
            Stmt::Return(expr) => {
                let t1 = expr.type_check(environment)?;
                if environment.return_lookup().eq(&t1) {
                    Ok(())
                } else if checks_empty_array(environment.return_lookup(), t1) {
                    return Ok(());
                } else {
                    Err(errors::ReturnTypeNotMatch(t1, environment.return_lookup()).into())
                }
            }
            Stmt::Draw { shape, point } => match point {
                Some(p) => {
                    let t1 = shape.type_check(environment)?;
                    let t2 = p.type_check(environment)?;
                    if t1 == Type::Shape && t2 == Type::Point {
                        Ok(())
                    } else if t1 == Type::Shape {
                        Err(errors::DrawTypeFault(Type::Point, t2).into())
                    } else {
                        Err(errors::DrawTypeFault(Type::Shape, t1).into())
                    }
                }
                None => {
                    let t1 = shape.type_check(environment)?;
                    if t1 == Type::Shape {
                        Ok(())
                    } else {
                        Err(errors::DrawTypeFault(Type::Shape, t1).into())
                    }
                }
            },
            Stmt::For {
                counter,
                from,
                to,
                body,
            } => {
                if environment.vtable_lookup(counter).is_ok() {
                    return Err(errors::ForLoopCounterDeclared().into());
                }

                let t1 = from.type_check(environment)?;
                let t2 = to.type_check(environment)?;

                if !(t1 == Type::Int && t2 == Type::Int) {
                    return Err(errors::ForLoopTypeError(t1, t2).into());
                }
                // Both lines are needed as the first clone does not clone over the environment
                let mut new_environment = environment.clone_and_clear_vtable();
                new_environment.clone_from(&environment);

                new_environment.vtable_set(counter.to_string(), Type::Int);

                for stmt in body {
                    stmt.type_check(&mut new_environment)?;
                }

                Ok(())
            }
            Stmt::Fork {
                branches,
                otherwise,
            } => {
                for (bool_expr, body) in branches {
                    let t1 = bool_expr.type_check(environment)?;
                    if bool_expr.type_check(environment)? != Type::Bool {
                        return Err(errors::ForkNotBooltypeError(t1).into());
                    }
                    let mut new_environment = environment.clone();
                    new_environment.clone_from(&environment);

                    for stmt in body {
                        stmt.type_check(&mut new_environment)?;
                    }
                }

                match otherwise {
                    Some(otherwise) => {
                        let mut new_environment = environment.clone();
                        new_environment.clone_from(&environment);
                        for stmt in otherwise {
                            stmt.type_check(&mut new_environment)?;
                        }

                        Ok(())
                    }
                    None => Ok(()),
                }
            }
        }
    }
}

fn checks_empty_array(array: Type, empty: Type) -> bool {
    if empty != Type::Empty {
        return false;
    }

    match array {
        Type::IntArray
        | Type::FloatArray
        | Type::BoolArray
        | Type::PathArray
        | Type::PointArray
        | Type::ShapeArray
        | Type::PolygonArray
        | Type::ColorArray => true,
        _ => false,
    }
}
