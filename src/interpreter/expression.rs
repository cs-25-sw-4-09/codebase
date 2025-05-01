use std::collections::btree_map::Range;

use crate::{interpreter::InterpretS, program::{expression::Expr, statement::Stmt}};

use super::InterpretE;

use crate::program::operators::binaryoperator::BinaryOperator;

impl InterpretE for Expr {
    fn interpret(
        &self,
        environment: &mut super::environment::IEnvironment)
        -> Result<Expr, Box<dyn std::error::Error>> {
        let expr = match self {
            Expr::Integer(_) | Expr::Boolean(_) | Expr::Float(_) | Expr::Color(_, _, _, _) => {
                self
            },
            Expr::Variable(_) => todo!(),
            Expr::BinaryOperation { lhs, rhs, operator } => {

                let i1 = lhs.interpret(environment)?;
                let i2 = rhs.interpret(environment)?;

                match operator {
                    BinaryOperator::Add => {
                        match (i1, i2) {
                            (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 + v2),
                            (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 + v2),
                            (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 + v2 as f64),
                            (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 + v2),
                            _ => unreachable!()
                        }
                    },
                    BinaryOperator::Subtract =>  {
                        match (i1, i2) {
                            (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 - v2),
                            (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 - v2),
                            (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 - v2 as f64),
                            (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 - v2),
                            _ => unreachable!()
                        }
                    },
                    BinaryOperator::Multiply => {
                        match (i1, i2) {
                            (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 * v2),
                            (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 * v2),
                            (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 * v2 as f64),
                            (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 * v2),
                            _ => unreachable!()
                        }
                    },
                    BinaryOperator::Divide => {
                        match (i1, i2) {
                            (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 / v2),
                            (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 / v2),
                            (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 / v2 as f64),
                            (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 / v2),
                            _ => unreachable!()
                        }
                    },
                    BinaryOperator::Modulus => {
                        match (i1, i2) {
                            (Expr::Integer(v1), Expr::Integer(v2)) => &Expr::Integer(v1 % v2),
                            (Expr::Float(v1), Expr::Float(v2)) => &Expr::Float(v1 % v2),
                            (Expr::Float(v1), Expr::Integer(v2)) => &Expr::Float(v1 % v2 as f64),
                            (Expr::Integer(v1), Expr::Float(v2)) => &Expr::Float(v1 as f64 % v2),
                            _ => unreachable!()
                        }
                    },
                    BinaryOperator::GreaterThanOrEquals => todo!(),
                    BinaryOperator::LessThanOrEquals => todo!(),
                    BinaryOperator::LessThan => todo!(),
                    BinaryOperator::GreaterThan => todo!(),
                    BinaryOperator::Equals => todo!(),
                    BinaryOperator::NotEquals => todo!(),
                    BinaryOperator::LogicalAnd => todo!(),
                    BinaryOperator::LogicalOr => todo!(),
                }
            },
            Expr::UnaryOperation { operator, expr } => todo!(),
            Expr::FCall { name, args } => {
                //Make new scope
                environment.push_scope();

                let function = environment.ftable_find(name.into()).unwrap();

                for i in 0..function.1.len() {
                    environment.vtable_push(function.1[i].clone(), args[i].clone());
                }
                
                for f in function.0 {
                    f.interpret(environment)?;
                }

                &environment.rvalue_get()

            },
            Expr::SCall { name, args } => todo!(),
        };
        Ok(expr.clone())
    }
}
