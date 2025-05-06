use crate::{interpreter::InterpretS, program::expression::Expr};

use super::{errors, InterpretE};

use crate::program::operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator};

impl InterpretE for Expr {
    fn interpret(
        &self,
        environment: &mut super::environment::IEnvironment,
    ) -> Result<Expr, Box<dyn std::error::Error>> {
        let expr = match self {
            Expr::Integer(_) | Expr::Boolean(_) | Expr::Float(_) | Expr::Color(_, _, _, _) => self,
            Expr::Variable(identifier) => environment.vtable_find(identifier.to_owned()).unwrap(),
            Expr::BinaryOperation { lhs, rhs, operator } => {
                let i1 = lhs.interpret(environment)?;
                let i2 = rhs.interpret(environment)?;

                match operator {
                    BinaryOperator::Add => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 + v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 + v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 + v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 + v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Subtract => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 - v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 - v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 - v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 - v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Multiply => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 * v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 * v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 * v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 * v2),
                        p => unreachable!("{:?}", p),
                    },
                    BinaryOperator::Divide => {
                        if i2 == Expr::Integer(0) || i2 == Expr::Float(0.0) {
                            return Err(errors::DivideByZero.into());
                        }
                        match (i1, i2) {
                            (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 / v2),
                            (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 / v2),
                            (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 / v2 as f64),
                            (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 / v2),
                            _ => unreachable!(),
                        }
                    }
                    BinaryOperator::Modulus => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 % v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 % v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 % v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 % v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::GreaterThanOrEquals => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 >= v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Boolean(v1 >= v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 >= v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Boolean(v1 as f64 >= v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LessThanOrEquals => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 <= v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Boolean(v1 <= v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 <= v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Boolean(v1 as f64 <= v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LessThan => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 < v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Boolean(v1 < v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 < v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Boolean((v1 as f64) < v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::GreaterThan => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 > v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Boolean(v1 > v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 > v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Boolean(v1 as f64 > v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Equals => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 == v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Boolean(v1 == v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 == v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Boolean(v1 as f64 == v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::NotEquals => match (i1, i2) {
                        (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 != v2),
                        (Expr::Float(v1), Expr::Float(v2)) => &Expr::Boolean(v1 != v2),
                        (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Boolean(v1 != v2 as f64),
                        (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Boolean(v1 as f64 != v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LogicalAnd => &Expr::Boolean(i1.get_bool()? && i2.get_bool()?),
                    BinaryOperator::LogicalOr => &Expr::Boolean(i1.get_bool()? || i2.get_bool()?),
                }
            }
            Expr::UnaryOperation { operator, expr } => {
                let i1 = expr.interpret(environment)?;
                match operator {
                    UnaryOperator::Negate => &Expr::Boolean(!i1.get_bool()?),
                    UnaryOperator::Negative => match i1 {
                        Expr::Integer(v) => &Expr::Integer(-v),
                        Expr::Float(v) => &Expr::Float(-v),
                        _ => unreachable!(),
                    },
                }
            }
            Expr::FCall { name, args } => {
                let mut params = Vec::new();
                let function = environment.ftable_find(name.into()).unwrap().clone();

                for i in 0..function.1.len() {
                    let i1 = args[i].clone().interpret(environment)?;
                    params.push((function.1[i].clone(), i1));
                }

                //Make new scope
                let previous_stack = environment.vtable_clear();

                for (param_identifier, param_elem) in params {
                    environment.vtable_push(param_identifier, param_elem);
                }

                for f in function.0 {
                    f.interpret(environment)?;
                }
                //todo: push ftable and pop
                //Restore scope
                environment.vtable_restore(previous_stack);

                if let Some(rvalue) = environment.rvalue_get() {
                    environment.rvalue_clear();
                    &rvalue.interpret(environment)?
                } else {
                    return Err(errors::FunctionNotReturning(name.to_owned()).into());
                }
            }
            Expr::Point(expr, expr1) => todo!(),
            Expr::PathOperation { lhs, rhs, operator } => todo!(),
            Expr::PolygonOperation { path, operator } => todo!(),
            Expr::Array(exprs) => todo!(),
            Expr::SCall {
                name,
                args,
                path_poly,
            } => todo!(),
            Expr::Member {
                identifier,
                member_access,
            } => todo!(),
            Expr::Place {
                base_shape,
                second_shape,
                place_at,
                point,
            } => todo!(),
            Expr::Scale { base_shape, factor } => todo!(),
            Expr::Rotate { base_shape, factor } => todo!(),
        };
        Ok(expr.clone())
    }
}
