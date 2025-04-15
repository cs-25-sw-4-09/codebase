use std::error::Error;

use crate::program::{
    expression::Expr,
    operators::{
        binaryoperator::BinaryOperator, pathoperator::PathOperator, unaryoperator::UnaryOperator,
    },
    r#type::Type,
};

use super::{environment::TEnvironment, errors, TypeCheckE};

impl TypeCheckE for Expr {
    fn type_check(
        &self,
        environment: &mut TEnvironment,
    ) -> Result<crate::program::r#type::Type, Box<dyn Error>> {
        match self {
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Point(x, y) => {
                let t1 = x.type_check(environment)?;
                let t2 = y.type_check(environment)?;

                match (t1, t2) {
                    (Type::Int, Type::Int)
                    | (Type::Float, Type::Int)
                    | (Type::Int, Type::Float)
                    | (Type::Float, Type::Float) => Ok(Type::Point),
                    _ => Err(errors::PointTypeNotCompatible(t1, t2).into()),
                }
            }
            Expr::Color(r, g, b, a) => {
                let t1 = r.type_check(environment)?;
                let t2 = g.type_check(environment)?;
                let t3 = b.type_check(environment)?;
                let t4 = a.type_check(environment)?;

                match (t1, t2, t3, t4) {
                    (Type::Int, Type::Int, Type::Int, Type::Int) => Ok(Type::Color),
                    _ => Err(errors::ColorTypeNotCompatible(t1, t2, t3, t4).into()),
                }
            }
            Expr::PathOperation { lhs, rhs, operator } => {
                let t1 = lhs.type_check(environment)?;
                let t2 = rhs.type_check(environment)?;

                match operator {
                    PathOperator::Line | PathOperator::Curve => match (t1, t2) {
                        (Type::Point, Type::Point)
                        | (Type::Path, Type::Path)
                        | (Type::Point, Type::Path)
                        | (Type::Path, Type::Point) => Ok(Type::Path),
                        (t1, t2) => Err(errors::PathOperationTypeNotCompatible(t1, t2).into()),
                    },
                }
            }
            Expr::Variable(identifier) => environment
                .vtable_lookup(identifier)
                .cloned()
                .ok_or(errors::IdentifierNotFound(identifier.to_owned()).into()),
            Expr::BinaryOperation { lhs, rhs, operator } => {
                let t1 = lhs.type_check(environment)?;
                let t2 = rhs.type_check(environment)?;
                match operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Divide
                    | BinaryOperator::Multiply
                    | BinaryOperator::Modulus => match (t1, t2) {
                        (Type::Int, Type::Int) => Ok(Type::Int),
                        (Type::Float, Type::Float)
                        | (Type::Int, Type::Float)
                        | (Type::Float, Type::Int) => Ok(Type::Float),
                        (t1, t2) => Err(errors::BinaryOperationTypeNotCompatible(t1, t2).into()),
                    },
                    BinaryOperator::LessThan
                    | BinaryOperator::LessThanOrEquals
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::GreaterThanOrEquals
                    | BinaryOperator::NotEquals
                    | BinaryOperator::Equals => match (t1, t2) {
                        (Type::Int, Type::Int)
                        | (Type::Float, Type::Int)
                        | (Type::Int, Type::Float)
                        | (Type::Float, Type::Float) => Ok(Type::Bool),
                        (t1, t2) => Err(errors::BinaryOperationTypeNotCompatible(t1, t2).into()),
                    },
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                        if t1.eq(&Type::Bool) && t2.eq(&Type::Bool) {
                            Ok(Type::Bool)
                        } else {
                            Err(errors::BinaryOperationTypeNotCompatible(t1, t2).into())
                        }
                    }
                }
            }
            Expr::UnaryOperation { operator, expr } => {
                let t1 = expr.type_check(environment)?;
                match operator {
                    UnaryOperator::Negate => {
                        if t1.ne(&Type::Bool) {
                            Err(errors::UnaryOperationTypeNotCompatible(t1).into())
                        } else {
                            Ok(Type::Bool)
                        }
                    }
                }
            }
            Expr::FCall { name, args } => {
                if environment.ftable_contains(name) {
                    let (parameters, return_type) = environment
                        .ftable_lookup(name)
                        .ok_or_else(|| Box::new(errors::IdentifierNotFound(name.to_owned())))?
                        .clone();

                    if parameters.iter().zip(args).all(|(parameter_type, arg)| {
                        match arg.type_check(environment) {
                            Ok(t1) => t1.eq(parameter_type),
                            Err(_) => false,
                        }
                    }) {
                        Ok(return_type.clone())
                    } else {
                        Err(errors::FCallParametersIncompatible(name.to_owned()).into())
                    }
                } else {
                    Err(errors::IdentifierNotFound(name.to_owned()).into())
                }
            }
            Expr::SCall { name, args } => {
                //Type checks the Shape call
                if environment.stable_contains(name) {
                    let expected_types = environment.stable_lookup(name).ok_or(()).unwrap().clone();

                    for (key, value) in args.iter() {
                        if !expected_types.contains_key(key) {
                            return Err(errors::SCallParameterNotFound(
                                key.to_owned().into(),
                                name.to_owned().into(),
                            )
                            .into());
                        }

                        let t1 = value.type_check(environment)?;

                        if t1 != *expected_types.get(key).unwrap() {
                            return Err(errors::SCallParametersIncompatible(
                                name.to_owned(),
                                key.clone(),
                                expected_types.get(key).unwrap().clone(),
                                t1,
                            )
                            .into());
                        }
                    }

                    Ok(Type::Shape)
                } else {
                    Err(errors::IdentifierNotFound(name.to_owned()).into())
                }
            }
        }
    }
}
