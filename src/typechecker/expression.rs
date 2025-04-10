use crate::program::{
    expression::Expr,
    operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator},
    r#type::Type,
};

use super::{environment::TEnvironment, TypeCheckE};

impl TypeCheckE for Expr {
    fn type_check(
        &self,
        environment: &mut TEnvironment,
    ) -> Result<crate::program::r#type::Type, ()> {
        match self {
            Expr::Integer(_) => Ok(Type::Int),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::Float(_) => Ok(Type::Float),
            Expr::Variable(identifier) => environment.vtable_lookup(identifier).cloned().ok_or(()),
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
                        _ => Err(()),
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
                        _ => Err(()),
                    },
                    BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                        if t1.eq(&Type::Bool) && t2.eq(&Type::Bool) {
                            Ok(Type::Bool)
                        } else {
                            Err(())
                        }
                    }
                }
            }
            Expr::UnaryOperation { operator, expr } => {
                let t1 = expr.type_check(environment)?;
                match operator {
                    UnaryOperator::Negate => {
                        if t1.ne(&Type::Bool) {
                            Err(())
                        } else {
                            Ok(Type::Bool)
                        }
                    }
                }
            }
            Expr::FCall { name, args } => {
                if environment.ftable_contains(name) {
                    let (parameters, return_type) = environment.ftable_lookup(name).ok_or(())?;
                    
                    if parameters.iter().zip(args).all(|((_, parameter_type), arg)| {
                        if let Ok(t1) = arg.type_check(environment) {
                            t1.eq(parameter_type)
                        } else {
                            false
                        }
                    }) {
                        Ok(return_type.clone())
                    } else {
                        Err(())
                    }

                } else {
                    Err(())
                }
            },
        }
    }
}
