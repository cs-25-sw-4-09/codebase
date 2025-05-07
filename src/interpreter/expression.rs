use std::collections::btree_map::Values;

use crate::{interpreter::InterpretS, program::expression::Expr};

use super::{data_types::point::Point, errors, utils::path::path_to_fig, value::Value, InterpretE};

use crate::program::operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator};


impl InterpretE for Expr {
    fn interpret(
        &self,
        environment: &mut super::environment::IEnvironment,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let expr = match self {
            Expr::Integer(val) => &Value::Integer(*val),
            Expr::Boolean(val) => &Value::Boolean(*val),
            Expr::Float(val) => &Value::Float(*val),
            Expr::Color(r,g,b,a) => &Value::Color(
                Box::new(r.interpret(environment)?), 
                Box::new(g.interpret(environment)?), 
                Box::new(b.interpret(environment)?), 
                Box::new(a.interpret(environment)?)
            ), 
            Expr::Point(x, y) => &Value::Point(
                Point::from((x.interpret(environment)?, y.interpret(environment)?))
            ),
            Expr::Variable(identifier) => environment.vtable_find(identifier.to_owned()).unwrap(),
            Expr::BinaryOperation { lhs, rhs, operator } => {
                let i1 = lhs.interpret(environment)?;
                let i2 = rhs.interpret(environment)?;

                match operator {
                    BinaryOperator::Add => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Integer(v1 + v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Float(v1 + v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Float(v1 + v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Float(v1 as f64 + v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Subtract => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Integer(v1 - v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Float(v1 - v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Float(v1 - v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Float(v1 as f64 - v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Multiply => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Integer(v1 * v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Float(v1 * v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Float(v1 * v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Float(v1 as f64 * v2),
                        p => unreachable!("{:?}", p),
                    },
                    BinaryOperator::Divide => {
                        if i2 == Value::Integer(0) || i2 == Value::Float(0.0) {
                            return Err(errors::DivideByZero.into());
                        }
                        match (i1, i2) {
                            (Value::Integer(v1), Value::Integer(v2)) => &Value::Integer(v1 / v2),
                            (Value::Float(v1), Value::Float(v2)) => &Value::Float(v1 / v2),
                            (Value::Float(v1), Value::Integer(v2)) => &Value::Float(v1 / v2 as f64),
                            (Value::Integer(v1), Value::Float(v2)) => &Value::Float(v1 as f64 / v2),
                            _ => unreachable!(),
                        }
                    }
                    BinaryOperator::Modulus => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Integer(v1 % v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Float(v1 % v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Float(v1 % v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Float(v1 as f64 % v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::GreaterThanOrEquals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Boolean(v1 >= v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Boolean(v1 >= v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Boolean(v1 >= v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Boolean(v1 as f64 >= v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LessThanOrEquals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Boolean(v1 <= v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Boolean(v1 <= v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Boolean(v1 <= v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Boolean(v1 as f64 <= v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LessThan => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Boolean(v1 < v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Boolean(v1 < v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Boolean(v1 < v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Boolean((v1 as f64) < v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::GreaterThan => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Boolean(v1 > v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Boolean(v1 > v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Boolean(v1 > v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Boolean(v1 as f64 > v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Equals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Boolean(v1 == v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Boolean(v1 == v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Boolean(v1 == v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Boolean(v1 as f64 == v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::NotEquals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => &Value::Boolean(v1 != v2),
                        (Value::Float(v1), Value::Float(v2)) => &Value::Boolean(v1 != v2),
                        (Value::Float(v1), Value::Integer(v2)) => &Value::Boolean(v1 != v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => &Value::Boolean(v1 as f64 != v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LogicalAnd => &Value::Boolean(i1.get_bool()? && i2.get_bool()?),
                    BinaryOperator::LogicalOr => &Value::Boolean(i1.get_bool()? || i2.get_bool()?),
                }
            }
            Expr::UnaryOperation { operator, expr } => {
                let i1 = expr.interpret(environment)?;
                match operator {
                    UnaryOperator::Negate => &Value::Boolean(!i1.get_bool()?),
                    UnaryOperator::Negative => match i1 {
                        Value::Integer(v) => &Value::Integer(-v),
                        Value::Float(v) => &Value::Float(-v),
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
                    //todo: potentielt kom tilbage
                    &rvalue.clone()
                } else {
                    return Err(errors::FunctionNotReturning(name.to_owned()).into());
                }
            }
            Expr::PathOperation { lhs:_, rhs:_,  operator:_ } => {
                let i1 = path_to_fig(self, environment)?;
                &Value::Path(i1)
            }
            Expr::PolygonOperation { path, operator } => {
                use crate::program::operators::polyoperator::PolyOperator;

                let mut i1 = match path.interpret(environment)? {
                    Value::Path(figure) => figure,
                    _ => return Err(Box::new(errors::PolyPathNotFound))
                };
                match operator {
                    PolyOperator::Curved => {
                        let first_point = i1.get_lines()[0].get_points()[0].clone();
                        let len = i1.get_lines().len();

                        i1.get_mut_line(len - 1).map(|l| l.push_point(first_point));
                    },
                    PolyOperator::Straight => {
                        let first_point = i1.get_lines()[0].get_points()[0].clone();
                        let last_point = i1.get_lines().iter().last().unwrap().get_points().iter().last().unwrap().clone();

                        i1.push_points(vec![first_point, last_point]);
                    }
                }

                &Value::Polygon(i1)
            }
            Expr::Array(exprs) => {
                let mut values: Vec<Box<Value>> = Vec::new();
                for expr in exprs {
                    values.push(Box::new(expr.interpret(environment)?));
                }
                &Value::Array(values)
            },
            Expr::SCall {
                name,
                args,
                path_poly,
            } => {
                match (name, path_poly) {
                    (Some(_), _) => {
                        /*let shape = environment.vtable_find(name.into()).unwrap().clone();
                        for i in 0..args.len() { 
                            let i1 = args[i].clone().interpret(environment)?;                      
                        }*/
                    },
                    (None, Some(path)) => {
                        let i1 = path.interpret(environment)?;
                        

                    },
                    _ => return Err(Box::new(errors::PolyPathNotFound)),
                }        
                todo!()   
            }
            Expr::Member {
                identifier,
                member_access,
            } => {
                /*let t1 = *environment.vtable_find(*identifier).unwrap();
                match t1 {
                    Value::Color(r,g,b,a) => match member_access.as_str() {
                        "r" => &r,
                        "g" => &g,
                        "b" => &b,
                        "a" => &a,
                        _ => unreachable!()
                    }
                    Value::Point(point) =>  match member_access.as_str(){
                        "x" => &point.0,
                        "y" => &point.1,
                        _ => unreachable!()
                    },
                    Value::Shape(figures) => todo!(),
                    Value::Path(figure) => todo!(),
                    Value::Polygon(figure) => todo!(),
                    //todo kan man få højten og breden af en path
                    _ => ()
                }*/
                todo!()
        }
            
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
