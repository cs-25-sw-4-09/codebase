use std::error::Error;

use crate::program::{
    expression::Expr,
    operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator},
    r#type::Type,
};

use super::{environment::TEnvironment, errors::{self, IdentifierNotFound}, TypeCheckE};

impl TypeCheckE for Expr {
    fn type_check(
        &self,
        environment: &mut TEnvironment,
    ) -> Result<crate::program::r#type::Type, Box<dyn Error>> {
        match self {
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Variable(identifier) => environment.vtable_lookup(identifier).cloned().ok_or(errors::IdentifierNotFound(identifier).into()),
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
                        (t1,t2) => Err(errors::BinaryOperationTypeNotCompatible(t1, t2).into()),
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
                        (t1,t2) => Err(errors::BinaryOperationTypeNotCompatible(t1, t2).into()),
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
                    let (parameters, return_type) =
                        environment.ftable_lookup(name).ok_or::<IdentifierNotFound>(errors::IdentifierNotFound(name).into())?.clone();

                    if parameters.iter().zip(args).all(|(parameter_type, arg)| {
                        match arg.type_check(environment) {
                            Ok(t1) => t1.eq(parameter_type),
                            Err(_) => false,
                        }
                    }) {
                        Ok(return_type.clone())
                    } else {
                        Err(errors::FCallParametersIncompatible(name).into())
                    }
                } else {
                    Err(errors::IdentifierNotFound(name).into())
                }
            }
            Expr::SCall { name, args } => {
                if environment.stable_contains(name) {
                    let expected_types = environment.stable_lookup(name).ok_or(()).unwrap().clone();

                    if args.iter().all(|(key, value)| {
                        match value.type_check(environment) {
                            Ok(t1) => t1.eq(expected_types.get(key).unwrap()),
                            Err(_) => false,
                        }
                    }) {
                        Ok(Type::Shape)
                    } else {
                        Err(errors::SCallParametersIncompatible(name).into())
                    }
                } else {
                    Err(errors::IdentifierNotFound(name).into())
                }
            },
        }
    }
}
