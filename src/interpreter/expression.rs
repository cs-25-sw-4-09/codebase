
use crate::{
    interpreter::{data_types::line::Line, InterpretS},
    program::expression::Expr,
};

use super::{
    data_types::{figurearray::FigureArray, point::Point},
    errors,
    utils::manipulation::{place, rotate, scale},
    value::Value,
    InterpretE, InterpretP,
};

use crate::program::operators::{
    binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
    unaryoperator::UnaryOperator,
};

impl InterpretE for Expr {
    fn interpret(
        &self,
        environment: &mut super::environment::IEnvironment,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let val = match self {
            Expr::Integer(val) => Value::Integer(*val),
            Expr::Boolean(val) => Value::Boolean(*val),
            Expr::Float(val) => Value::Float(*val),
            Expr::Color(r, g, b, a) => Value::Color(
                Box::new(r.interpret(environment)?),
                Box::new(g.interpret(environment)?),
                Box::new(b.interpret(environment)?),
                Box::new(a.interpret(environment)?),
            ),
            Expr::Point(x, y) => Value::Point(Point::from((
                x.interpret(environment)?,
                y.interpret(environment)?,
            ))),
            Expr::Variable(identifier) => environment.vtable_find(identifier.to_owned()).unwrap().clone(),
            Expr::BinaryOperation { lhs, rhs, operator } => {
                let i1 = lhs.interpret(environment)?;
                let i2 = rhs.interpret(environment)?;

                match operator {
                    BinaryOperator::Add => (i1 + i2),
                    BinaryOperator::Subtract => (i1 - i2),
                    BinaryOperator::Multiply =>  (&i1 * &i2),
                    BinaryOperator::Divide => {
                        if i2 == Value::Integer(0) || i2 == Value::Float(0.0) {
                            return Err(errors::DivideByZero.into());
                        }
                        &i1 / &i2
                    }
                    BinaryOperator::Modulus => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 % v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 % v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 % v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 % v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::GreaterThanOrEquals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Boolean(v1 >= v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Boolean(v1 >= v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Boolean(v1 >= v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Boolean(v1 as f64 >= v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LessThanOrEquals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Boolean(v1 <= v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Boolean(v1 <= v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Boolean(v1 <= v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Boolean(v1 as f64 <= v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LessThan => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Boolean(v1 < v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Boolean(v1 < v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Boolean(v1 < v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Boolean((v1 as f64) < v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::GreaterThan => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Boolean(v1 > v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Boolean(v1 > v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Boolean(v1 > v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Boolean(v1 as f64 > v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::Equals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Boolean(v1 == v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Boolean(v1 == v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Boolean(v1 == v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Boolean(v1 as f64 == v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::NotEquals => match (i1, i2) {
                        (Value::Integer(v1), Value::Integer(v2)) => Value::Boolean(v1 != v2),
                        (Value::Float(v1), Value::Float(v2)) => Value::Boolean(v1 != v2),
                        (Value::Float(v1), Value::Integer(v2)) => Value::Boolean(v1 != v2 as f64),
                        (Value::Integer(v1), Value::Float(v2)) => Value::Boolean(v1 as f64 != v2),
                        _ => unreachable!(),
                    },
                    BinaryOperator::LogicalAnd => Value::Boolean(i1.get_bool()? && i2.get_bool()?),
                    BinaryOperator::LogicalOr => Value::Boolean(i1.get_bool()? || i2.get_bool()?),
                }
            }
            Expr::UnaryOperation { operator, expr } => {
                let i1 = expr.interpret(environment)?;
                match operator {
                    UnaryOperator::Negate => Value::Boolean(!i1.get_bool()?),
                    UnaryOperator::Negative => -i1,
                }
            }
            Expr::FCall { name, args } => {
                match name.as_str() {
                    "push" => {
                        let i1 = args[0].interpret(environment)?;
                        let i2 = args[1].interpret(environment)?;
                        let mut array = i1.get_array()?;
                        array.push(Box::new(i2));
                        Value::Array(array)
                    }
                    "remove" => {
                        let i1 = args[0].interpret(environment)?;
                        let i2 = args[1].interpret(environment)?;
                        let index_to_remove = i2.get_int()? as usize;
                        let mut array = i1.get_array()?;
                        array.remove(index_to_remove);
                        Value::Array(array)
                    }
                    _ => {
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

                        let rvalue = environment.rvalue_get()
                        .ok_or_else(|| Box::new(errors::FunctionNotReturning(name.to_owned())))?
                        .clone();
                        environment.rvalue_clear();
                        rvalue
                    }
                }
            }
            Expr::PathOperation { lhs, rhs, operator } => {
                let i1 = lhs.interpret(environment)?;
                let i2 = rhs.interpret(environment)?;

                match (i1, i2) {
                    //Path-path
                    (Value::Figure(mut fig1), Value::Figure(mut fig2)) => {
                        let fig1_last_line = fig1.get_last_line()?;
                        let fig2_first_line = fig2.get_first_line()?;

                        match (operator, &fig1_last_line, &fig2_first_line) {
                            (PathOperator::Line, _, _)
                            | (PathOperator::Curve, Line::Straight(_), Line::Straight(_)) => {
                                let fig1_last_line = fig1_last_line.clone();
                                fig1.push_line_after(Line::Straight(vec![
                                    fig1_last_line.get_last_point()?.clone(),
                                    fig2_first_line.get_first_point()?.clone(),
                                ]));
                                fig1.push_lines(fig2.get_lines().clone());
                                Value::Figure(fig1)
                            }
                            (PathOperator::Curve, Line::Straight(_), Line::Curved(_)) => {
                                fig2_first_line
                                    .insert_point_first(fig1_last_line.get_last_point()?.clone());

                                fig1.push_lines(fig2.get_lines().clone());
                                Value::Figure(fig1)
                            }
                            (PathOperator::Curve, Line::Curved(_), Line::Curved(_)) => {
                                let fig1_last_line = fig1.pop_last_line()?;
                                let fig2_first_line = fig2.pop_first_line()?;

                                fig1.push_line_after(Line::Curved(
                                    fig1_last_line
                                        .get_points()
                                        .to_vec()
                                        .into_iter()
                                        .chain(fig2_first_line.get_points().to_vec().into_iter())
                                        .collect(),
                                ));

                                fig1.push_lines(fig2.get_lines().clone());
                                Value::Figure(fig1)
                            }
                            (PathOperator::Curve, Line::Curved(_), Line::Straight(_)) => {
                                fig1_last_line
                                    .insert_point_last(fig2_first_line.get_first_point()?.clone());

                                fig1.push_lines(fig2.get_lines().clone());
                                Value::Figure(fig1)
                            }
                        }
                    }
                    //Point-path
                    (Value::Point(p), Value::Figure(mut fig)) => {
                        let line_first = fig.get_first_line()?;
                        match (operator, &line_first) {
                            (PathOperator::Line, _) => {
                                let line_first = line_first.clone();
                                fig.push_line_before(Line::Straight(vec![
                                    p.clone(),
                                    line_first.get_first_point()?.clone(),
                                ]));
                                Value::Figure(fig)
                            }
                            (PathOperator::Curve, Line::Curved(_)) => {
                                line_first.insert_point_first(p);
                                Value::Figure(fig)
                            }
                            (PathOperator::Curve, Line::Straight(_)) => {
                                let line_first = line_first.clone();
                                fig.push_line_before(Line::Curved(vec![
                                    p.clone(),
                                    line_first.get_first_point()?.clone(),
                                ]));
                                Value::Figure(fig)
                            }
                        }
                    }
                    //path-point
                    (Value::Figure(mut fig), Value::Point(p)) => {
                        let line_last = fig.get_last_line()?;
                        match (operator, &line_last) {
                            (PathOperator::Line, _) => {
                                let line_last = line_last.clone();
                                fig.push_line_after(Line::Straight(vec![
                                    line_last.get_last_point()?.clone(),
                                    p.clone(),
                                ]));
                                Value::Figure(fig)
                            }
                            (PathOperator::Curve, Line::Curved(_)) => {
                                line_last.insert_point_last(p);
                                Value::Figure(fig)
                            }
                            (PathOperator::Curve, Line::Straight(_)) => {
                                let line_last = line_last.clone();
                                fig.push_line_after(Line::Curved(vec![
                                    line_last.get_last_point()?.clone(),
                                    p.clone(),
                                ]));
                                Value::Figure(fig)
                            }
                        }
                    }
                    //point-point
                    (Value::Point(p1), Value::Point(p2)) => match operator {
                        PathOperator::Line => Value::Figure(
                            vec![Line::Straight(vec![p1.clone(), p2.clone()])].into(),
                        ),
                        PathOperator::Curve => {
                            Value::Figure(vec![Line::Curved(vec![p1.clone(), p2.clone()])].into())
                        }
                    },
                    _ => unreachable!(),
                }
            }
            Expr::PolygonOperation { path, operator } => {
                let i1 = path.interpret(environment)?;

                let Value::Figure(mut fig) = i1 else {
                    unreachable!()
                };

                let line_first = fig.get_first_line()?.clone();
                let line_last = fig.get_last_line()?;

                match (operator, &line_first, &line_last) {
                    (PolyOperator::Curved, Line::Straight(_), Line::Curved(_)) => {
                        // Case 1
                        line_last.insert_point_last(line_first.get_first_point()?.clone());
                        Value::Figure(fig)
                    }
                    (PolyOperator::Curved, Line::Curved(_), Line::Straight(_)) => {
                        // NY CASE: istedet for at have den ved case 2 hvor e.g. (a,b)~~(c,d)--(e,f)~~* => (c,d)--(e,f) ville blive konverteret til (c,d)~~(e,f)
                        let mut line_first = fig.pop_first_line()?;
                        line_first
                            .insert_point_first(fig.get_last_line()?.get_last_point()?.clone());
                        fig.push_line_after(line_first);
                        Value::Figure(fig)
                    }

                    (PolyOperator::Curved, Line::Curved(_), Line::Curved(_)) => {
                        // Case 2
                        if fig.get_lines().len() > 1 {
                            let line_first = fig.pop_first_line()?;
                            let line_last = fig.pop_last_line()?;

                            fig.push_line_after(Line::Curved(
                                line_last
                                    .get_points()
                                    .to_vec()
                                    .into_iter()
                                    .chain(line_first.get_points().to_vec().into_iter())
                                    .collect(),
                            ));
                        } else {
                            let point_first = fig.get_first_line()?.get_first_point()?.clone();
                            fig.get_first_line()?.insert_point_last(point_first);
                        }
                        Value::Figure(fig)
                    }
                    (PolyOperator::Straight, _, _)
                    | (PolyOperator::Curved, Line::Straight(_), Line::Straight(_)) => {
                        // Case 3
                        let line_last = line_last.clone();
                        fig.push_line_after(Line::Straight(vec![
                            line_last.get_last_point()?.clone(),
                            line_first.get_first_point()?.clone(),
                        ]));
                        Value::Figure(fig)
                    }
                }
            }
            Expr::Array(exprs) => {
                let mut values: Vec<Box<Value>> = Vec::new();
                for expr in exprs {
                    values.push(Box::new(expr.interpret(environment)?));
                }
                Value::Array(values)
            }
            Expr::SCall {
                name,
                args,
                path_poly,
            } => {
                match (name, path_poly) {
                    (Some(_), _) => {
                        let mut interpreted_args = Vec::new();
                        for (arg_name, expr) in args.iter() {
                            let i1 = expr.interpret(environment)?;
                            interpreted_args.push((arg_name, i1));
                        }

                        let mut program = environment
                            .stable_find(name.clone().unwrap())
                            .unwrap()
                            .clone();

                        for (arg_name, value) in interpreted_args {
                            program.ienvironment.vtable_push(arg_name.clone(), value);
                        }

                        let draw_array = match program.interpret() {
                            Ok(draw_array) => {
                                println!(
                                    "[Interpreter] Constructor Call: {} - OK",
                                    name.clone().unwrap()
                                );
                                draw_array
                            }
                            Err(err) => {
                                println!(
                                    "[Interpreter] Constructor Call: {} - ERROR",
                                    name.clone().unwrap()
                                );
                                return Err(err);
                            }
                        };

                        Value::Shape(FigureArray::from(draw_array.clone()))
                    }
                    (None, Some(path_poly)) => {
                        //Shape call to path/polygon
                        let i1 = path_poly.interpret(environment)?;

                        let Value::Figure(mut fig) = i1 else {
                            unreachable!()
                        };

                        for (arg_name, expr) in args.clone() {
                            fig.set_attribute((arg_name, expr.interpret(environment)?));
                        }

                        Value::Shape(FigureArray::from(vec![fig]))
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Member {
                identifier,
                member_access,
            } => {
                let t1 = environment.vtable_find(identifier.into()).unwrap().clone();
                match t1 {
                    Value::Color(r, g, b, a) => match member_access.as_str() {
                        "r" => *r,
                        "g" => *g,
                        "b" => *b,
                        "a" => *a,
                        _ => unreachable!(),
                    },
                    Value::Point(point) => match member_access.as_str() {
                        "x" => point.get_x().clone(),
                        "y" => point.get_y().clone(),
                        _ => unreachable!(),
                    },
                    Value::Shape(figures) => match member_access.as_str() {
                        "height" => figures.height(),
                        "width" => figures.width(),
                        "x" => figures.get_top_left().get_x().clone(),
                        "y" => figures.get_top_left().get_y().clone(),
                        _ => unreachable!(),
                    },
                    Value::Figure(figure) => match member_access.as_str() {
                        "height" => figure.get_height(),
                        "width" => figure.get_width(),
                        _ => unreachable!(),
                    },
                    Value::Array(array) => match member_access.as_str() {
                        "size" => Value::Integer(array.len() as i64),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
            Expr::Place {
                base_shape,
                second_shape,
                place_at,
                point,
            } => {
                let (s1, s2, p, dir) = (
                    base_shape.interpret(environment)?.get_shape()?,
                    second_shape.interpret(environment)?.get_shape()?,
                    match point {
                        Some(exp) => exp.interpret(environment)?.get_point()?,
                        None => (0, 0).into(),
                    },
                    place_at.as_str().into(),
                );

                let v = place(s1, s2, p, dir);

                Value::Shape(v)
            }
            Expr::Scale { base_shape, factor } => {
                let Value::Shape(shape) = base_shape.interpret(environment)? else {
                    unreachable!()
                };
                let factor = factor.interpret(environment)?;
                let scaled_shape = scale(shape, factor)?;

                Value::Shape(scaled_shape)
            }
            Expr::Rotate { base_shape, factor } => {
                let s = base_shape.interpret(environment)?.get_shape()?;
                let i1 = factor.interpret(environment)?;
                let v = rotate(s, i1);
                Value::Shape(v)
            }
            Expr::ArrayIndex { identifier, index } => {
                let v1 = identifier.interpret(environment)?.get_array()?;
                let v2 = index.interpret(environment)?.get_int()?;

                let idx: usize = v2.try_into().map_err(|_| errors::ArrayOutOfBoundsWithNumbers(v1.len(), v2))?;
                *v1.get(idx).ok_or_else(|| errors::ArrayOutOfBoundsWithNumbers(v1.len(), v2))?.clone()
            },
        };

        Ok(val)
    }
}
