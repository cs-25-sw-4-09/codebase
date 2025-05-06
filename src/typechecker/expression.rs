use std::error::Error;

use crate::program::{
    expression::Expr,
    operators::{
        binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
        unaryoperator::UnaryOperator,
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
            Expr::Variable(identifier) => environment.vtable_lookup(identifier).cloned(),
            Expr::PathOperation { lhs, rhs, operator } => {
                                let t1 = lhs.type_check(environment)?;
                                let t2 = rhs.type_check(environment)?;

                                match operator {
                                    PathOperator::Line | PathOperator::Curve => match (t1, t2) {
                                        //implementation of typing rule "Path"
                                        (Type::Point, Type::Point)
                                        | (Type::Path, Type::Path)
                                        | (Type::Point, Type::Path)
                                        | (Type::Path, Type::Point) => Ok(Type::Path),
                                        //implementation of typing rule "extendPoly"
                                        (Type::Polygon, Type::Point)
                                        | (Type::Polygon, Type::Path)
                                        | (Type::Polygon, Type::Polygon)
                                        | (Type::Path, Type::Polygon)
                                        | (Type::Point, Type::Polygon) => Ok(Type::Polygon),
                                        (t1, t2) => Err(errors::PathOperationTypeNotCompatible(t1, t2).into()),
                                    },
                                }
                            }
            Expr::PolygonOperation { path, operator } => {
                                let t1 = path.type_check(environment)?;

                                match operator {
                                    PolyOperator::Straight | PolyOperator::Curved => match t1 {
                                        Type::Path => Ok(Type::Polygon),
                                        t1 => Err(errors::PolyOperationTypeNotCompatible(t1).into()),
                                    },
                                }
                            }
            Expr::Array(exprs) => {
                                //implements typing rule for empty arrays
                                if exprs.len() == 0 {
                                    Ok(Type::Empty)
                                } else {
                                    //implements typing rule for nonempty arrays
                                    let t_for_array = exprs[0].type_check(environment)?;
                                    exprs.iter().try_for_each(|expr| {
                                        let t = expr.type_check(environment)?;
                                        if t == t_for_array {
                                            Ok::<(), Box<dyn std::error::Error>>(())
                                        } else {
                                            Err(errors::ArrayElementsTypeNotCompatible(t_for_array.clone()).into())
                                        }
                                    })?;
                                    // If we get here, all elements' types matched
                                    match t_for_array {
                                        Type::Int => Ok(Type::IntArray),
                                        Type::Bool => Ok(Type::BoolArray),
                                        Type::Float => Ok(Type::FloatArray),
                                        Type::Shape => Ok(Type::ShapeArray),
                                        Type::Path => Ok(Type::PathArray),
                                        Type::Point => Ok(Type::PointArray),
                                        Type::Polygon => Ok(Type::PolygonArray),
                                        Type::Color => Ok(Type::ColorArray),
                                        _ => Err(errors::ArrayElementsTypeNotCompatible(t_for_array).into()),
                                    }
                                }
                            }
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
                                    UnaryOperator::Negative => {
                                        if t1.eq(&Type::Int) {
                                            Ok(Type::Int)
                                        } else if t1.eq(&Type::Float) {
                                            Ok(Type::Float)
                                        } else {
                                            Err(errors::UnaryOperationTypeNotCompatible(t1).into())
                                        }
                                    }
                                }
                            }
            Expr::FCall { name, args } => {
                                let (parameters, return_type) = environment.ftable_lookup(name)?.clone();

                                // Checks that the right amount of params is passed
                                if parameters.len() != args.len() {
                                    return Err(errors::FCallParametersCountError(name.to_owned()).into());
                                }
                

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
                            }
            Expr::SCall {
                                name,
                                args,
                                path_poly,
                            } => {
                                match name {
                                    Some(name) => {
                                        //Type checks the Shape call
                                        let expected_types = environment.stable_lookup(name)?.clone();

                                        for (key, value) in args.iter() {
                                            if !expected_types.contains_key(key) {
                                                return Err(errors::SCallParameterNotFound(
                                                    key.into(),
                                                    name.into(),
                                                )
                                                .into());
                                            }

                                            let t1 = value.type_check(environment)?;

                                            let shape_t = match *expected_types.get(key).unwrap() {
                                                super::environment::EType::DeclNonDefault(x)
                                                | super::environment::EType::DeclDefault(x) => x,
                                                super::environment::EType::Normal(_)=> unreachable!()
                                            };

                                            if t1 != shape_t {
                                                return Err(errors::SCallParametersIncompatible(
                                                    name.to_owned(),
                                                    key.clone(),
                                                    shape_t,
                                                    t1,
                                                )
                                                .into());
                                            }
                                        }
                                        // non default params are only checked on name, since type was chekced above.
                                        let non_def = environment.stable_get_hashmap_non_default(name)?;
                                        non_def.iter().try_for_each(|(key, _)| {
                                            if !args.contains_key(key) {
                                                Err(errors::SCallParameterNotFound(key.into(), name.into()))
                                            } else {
                                                Ok(())
                                            }
                                        })?;

                                        Ok(Type::Shape)
                                    }
                                    None => {
                                        let t1 = path_poly
                                            .as_ref()
                                            .ok_or_else(|| errors::PolyPathNotFound())?
                                            .type_check(environment)?;
                                        match t1 {
                                            Type::Path => {
                                                let scall = Expr::SCall {
                                                    name: Some("Path".to_string()),
                                                    args: args.clone(),
                                                    path_poly: None,
                                                };
                                                scall.type_check(environment)
                                            }
                                            Type::Polygon => {
                                                let scall = Expr::SCall {
                                                    name: Some("Polygon".to_string()),
                                                    args: args.clone(),
                                                    path_poly: None,
                                                };
                                                scall.type_check(environment)
                                            }
                                            _ => Err(errors::PolyPathNotFound().into()),
                                        }
                                    }
                                }
                            }
            Expr::Member {
                                identifier,
                                member_access,
                            } => {
                                let t1 = environment.vtable_lookup(identifier)?.clone();
                                match t1 {
                                    Type::Color => match member_access.as_str() {
                                        "r" => Ok(Type::Int),
                                        "g" => Ok(Type::Int),
                                        "b" => Ok(Type::Int),
                                        "a" => Ok(Type::Int),
                                        _ => Err(errors::MemberAccessColor().into()),
                                    },
                                    Type::Shape => match member_access.as_str() {
                                        "width" => Ok(Type::Float),
                                        "height" => Ok(Type::Float),
                                        _ => Err(errors::MemberAccessShape().into()),
                                    },
                                    Type::Point => match member_access.as_str() {
                                        "x" => Ok(Type::Float),
                                        "y" => Ok(Type::Float),
                                        _ => Err(errors::MemberAccessPoint().into()),
                                    },
                                    Type::BoolArray
                                    | Type::FloatArray
                                    | Type::IntArray
                                    | Type::ColorArray
                                    | Type::PathArray
                                    | Type::PointArray
                                    | Type::PolygonArray
                                    | Type::ShapeArray => match member_access.as_str() {
                                        "size" => Ok(Type::Int),
                                        _ => Err(errors::MemberAccessArray().into()),
                                    },
                                    _ => Err(errors::NotAMemberType(t1).into()),
                                }
                            }
            Expr::Place {
                                base_shape,
                                second_shape,
                                point,
                                ..
                            } => {
                                let t1 = base_shape.type_check(environment)?;
                                if t1 != Type::Shape {
                                    return Err(errors::ManipulationPlaceTypeFault(Type::Shape, t1).into());
                                }
                                let t2 = second_shape.type_check(environment)?;
                                if t2 != Type::Shape {
                                    return Err(errors::ManipulationPlaceTypeFault(Type::Shape, t2).into());
                                }
                                match point {
                                    Some(p) => {
                                        let t3 = p.type_check(environment)?;
                                        if t3 != Type::Point {
                                            return Err(errors::ManipulationPlaceTypeFault(Type::Point, t3).into());
                                        }
                                    }
                                    None => {}
                                }

                                Ok(Type::Shape)
                            }
            Expr::Scale { base_shape, factor } => {
                                let t1 = base_shape.type_check(environment)?;
                                let t2 = factor.type_check(environment)?;

                                if t1 == Type::Shape && (t2 == Type::Int || t2 == Type::Float) {
                                    Ok(Type::Shape)
                                } else {
                                    Err(errors::ManipulationScaleTypeFault(t1, t2).into())
                                }
                            }
            Expr::Rotate { base_shape, factor } => {
                                let t1 = base_shape.type_check(environment)?;
                                let t2 = factor.type_check(environment)?;

                                if t1 == Type::Shape && (t2 == Type::Int || t2 == Type::Float) {
                                    Ok(Type::Shape)
                                } else {
                                    Err(errors::ManipulationRotateTypeFault(t1, t2).into())
                                }
                            }
            Expr::Shape(_) => Ok(Type::Shape),
            Expr::Path(_) => Ok(Type::Path),
            Expr::Polygon(_) => Ok(Type::Polygon),
        }
    }
}
