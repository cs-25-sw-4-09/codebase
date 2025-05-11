use crate::{
    program::{
        expression::Expr,
        operators::{
            binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
            unaryoperator::UnaryOperator,
        },
        r#type::Type,
    },
    typechecker::{
        environment::{EType, TEnvironment},
        errors, TypeCheckE,
    },
};

#[test]
fn integer() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Integer(4).type_check(&mut env).unwrap();
    assert_eq!(t1, Type::Int)
}

#[test]
fn boolean() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Boolean(true).type_check(&mut env).unwrap();
    assert_eq!(t1, Type::Bool)
}

#[test]
fn float() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Float(4.3).type_check(&mut env).unwrap();
    assert_eq!(t1, Type::Float)
}

#[test]
fn color() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Color(
        Box::new(Expr::Integer(1)),
        Box::new(Expr::Integer(2)),
        Box::new(Expr::Integer(3)),
        Box::new(Expr::Integer(4)),
    )
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Color)
}

#[test]
fn color_invalid() {
    let mut env = TEnvironment::new();
    let invalid = Expr::Color(
        Box::new(Expr::Integer(1)),
        Box::new(Expr::Integer(2)),
        Box::new(Expr::Integer(3)),
        Box::new(Expr::Float(4.2)),
    )
    .type_check(&mut env);
    assert!(invalid
        .unwrap_err()
        .downcast_ref::<errors::ColorTypeNotCompatible>()
        .is_some());
}

#[test]
fn point() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Point(Box::new(Expr::Integer(1)), Box::new(Expr::Integer(2)))
        .type_check(&mut env)
        .unwrap();
    assert_eq!(t1, Type::Point)
}

#[test]
fn point_invalid() {
    let mut env = TEnvironment::new();
    let invalid =
        Expr::Point(Box::new(Expr::Integer(1)), Box::new(Expr::Boolean(true))).type_check(&mut env);
    assert!(invalid
        .unwrap_err()
        .downcast_ref::<errors::PointTypeNotCompatible>()
        .is_some());
}

#[test]
fn path_with_points() {
    let mut env = TEnvironment::new();
    let t1 = Expr::PathOperation {
        lhs: Box::new(Expr::Point(
            Box::new(Expr::Integer(1)),
            Box::new(Expr::Integer(2)),
        )),
        rhs: Box::new(Expr::Point(
            Box::new(Expr::Integer(3)),
            Box::new(Expr::Integer(4)),
        )),
        operator: PathOperator::Line,
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Path)
}

#[test]
fn path_with_path_and_point() {
    let mut env = TEnvironment::new();
    let t1 = Expr::PathOperation {
        lhs: Box::new(Expr::PathOperation {
            lhs: Box::new(Expr::Point(
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            )),
            rhs: Box::new(Expr::Point(
                Box::new(Expr::Integer(3)),
                Box::new(Expr::Integer(4)),
            )),
            operator: PathOperator::Line,
        }),
        rhs: Box::new(Expr::Point(
            Box::new(Expr::Integer(3)),
            Box::new(Expr::Integer(4)),
        )),
        operator: PathOperator::Line,
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Path)
}

#[test]
fn path_invalid() {
    let mut env = TEnvironment::new();
    let invalid = Expr::PathOperation {
        lhs: Box::new(Expr::Integer(1)),
        rhs: Box::new(Expr::Point(
            Box::new(Expr::Integer(3)),
            Box::new(Expr::Integer(4)),
        )),
        operator: PathOperator::Line,
    }
    .type_check(&mut env);
    assert!(invalid
        .unwrap_err()
        .downcast_ref::<errors::PathOperationTypeNotCompatible>()
        .is_some());
}

#[test]
fn polygon() {
    let mut env = TEnvironment::new();
    let t1 = Expr::PolygonOperation {
        path: Box::new(Expr::PathOperation {
            lhs: Box::new(Expr::Point(
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            )),
            rhs: Box::new(Expr::Point(
                Box::new(Expr::Integer(3)),
                Box::new(Expr::Integer(4)),
            )),
            operator: PathOperator::Line,
        }),
        operator: PolyOperator::Straight,
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Polygon)
}

#[test]
fn polygon_invalid() {
    let mut env = TEnvironment::new();
    let invalid = Expr::PolygonOperation {
        path: Box::new(Expr::Integer(1)),
        operator: PolyOperator::Straight,
    }
    .type_check(&mut env);
    assert!(invalid
        .unwrap_err()
        .downcast_ref::<errors::PolyOperationTypeNotCompatible>()
        .is_some());
}

#[test]
fn array_with_elements() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Array(vec![Expr::Integer(1), Expr::Integer(1), (Expr::Integer(1))])
        .type_check(&mut env)
        .unwrap();
    assert_eq!(t1, Type::IntArray)
}

#[test]
fn array_empty() {
    let mut env = TEnvironment::new();
    let t1 = Expr::Array(vec![]).type_check(&mut env).unwrap();
    assert_eq!(t1, Type::Empty)
}

#[test]
fn array_invalid() {
    let mut env = TEnvironment::new();
    let invalid = Expr::Array(vec![Expr::Integer(1), Expr::Integer(1), (Expr::Float(1.2))])
        .type_check(&mut env);
    assert!(invalid
        .unwrap_err()
        .downcast_ref::<errors::ArrayElementsTypeNotCompatible>()
        .is_some());
}

#[test]
fn variable() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::Int);
    let t1 = Expr::Variable("x".into()).type_check(&mut env).unwrap();
    assert_eq!(t1, Type::Int)
}

#[test]
fn variable_invalid_identifier() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::Variable("x".into()).type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::IdentifierNotFound>()
        .is_some());
}

#[test]
fn binary_operation_arithmetic() {
    let mut env = TEnvironment::new();
    let tests: Vec<(Expr, Expr, Type)> = vec![
        (Expr::Integer(5), Expr::Integer(3), Type::Int),
        (Expr::Integer(5), Expr::Float(3.3), Type::Float),
        (Expr::Float(5.0), Expr::Integer(3), Type::Float),
        (Expr::Float(6.2), Expr::Float(3.3), Type::Float),
    ];
    for (lhs, rhs, expected) in tests {
        let t1 = Expr::BinaryOperation {
            lhs: lhs.into(),
            rhs: rhs.into(),
            operator: BinaryOperator::Add,
        }
        .type_check(&mut env)
        .unwrap();
        assert_eq!(t1, expected)
    }
}

#[test]
fn binary_operation_arithmetic_noncompatible() {
    let mut env = TEnvironment::new();
    let not_compatible = Expr::BinaryOperation {
        lhs: Expr::Integer(6).into(),
        rhs: Expr::Boolean(true).into(),
        operator: BinaryOperator::Add,
    }
    .type_check(&mut env);
    assert!(not_compatible
        .unwrap_err()
        .downcast_ref::<errors::BinaryOperationTypeNotCompatible>()
        .is_some());
}

#[test]
fn binary_operation_comparison() {
    let mut env = TEnvironment::new();
    let tests: Vec<(Expr, Expr, Type)> = vec![
        (Expr::Integer(5), Expr::Integer(3), Type::Bool),
        (Expr::Integer(5), Expr::Float(3.3), Type::Bool),
        (Expr::Float(5.0), Expr::Integer(3), Type::Bool),
        (Expr::Float(6.2), Expr::Float(3.3), Type::Bool),
    ];
    for (lhs, rhs, expected) in tests {
        let t1 = Expr::BinaryOperation {
            lhs: lhs.into(),
            rhs: rhs.into(),
            operator: BinaryOperator::LessThan,
        }
        .type_check(&mut env)
        .unwrap();
        assert_eq!(t1, expected)
    }
}

#[test]
fn binary_operation_comparison_noncompatible() {
    let mut env = TEnvironment::new();

    let not_compatible = Expr::BinaryOperation {
        lhs: Expr::Integer(6).into(),
        rhs: Expr::Boolean(true).into(),
        operator: BinaryOperator::LessThan,
    }
    .type_check(&mut env);
    assert!(not_compatible
        .unwrap_err()
        .downcast_ref::<errors::BinaryOperationTypeNotCompatible>()
        .is_some());
}

#[test]
fn binary_operation_logical() {
    let mut env = TEnvironment::new();
    let t1 = Expr::BinaryOperation {
        lhs: Expr::Boolean(true).into(),
        rhs: Expr::Boolean(false).into(),
        operator: BinaryOperator::LogicalAnd,
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Bool);
}

#[test]
fn binary_operation_logical_noncompatible() {
    let mut env = TEnvironment::new();

    let not_compatible = Expr::BinaryOperation {
        lhs: Expr::Integer(6).into(),
        rhs: Expr::Integer(3).into(),
        operator: BinaryOperator::LogicalAnd,
    }
    .type_check(&mut env);
    assert!(not_compatible
        .unwrap_err()
        .downcast_ref::<errors::BinaryOperationTypeNotCompatible>()
        .is_some());
}

#[test]
fn unary_operation() {
    let mut env = TEnvironment::new();
    let t1 = Expr::UnaryOperation {
        expr: Expr::Boolean(true).into(),
        operator: UnaryOperator::Negate,
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Bool);
}

#[test]
fn unary_operation_noncompatible() {
    let mut env = TEnvironment::new();

    let not_compatible = Expr::UnaryOperation {
        expr: Expr::Integer(6).into(),
        operator: UnaryOperator::Negate,
    }
    .type_check(&mut env);
    assert!(not_compatible
        .unwrap_err()
        .downcast_ref::<errors::UnaryOperationTypeNotCompatible>()
        .is_some());
}

#[test]
fn fcall() {
    let mut env = TEnvironment::new();
    env.ftable_set("x".into(), vec![Type::Int], Type::Bool);
    let t1 = Expr::FCall {
        name: "x".into(),
        args: vec![Expr::Integer(5)],
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Bool);
}



#[test]
fn fcall_invalid_identifier() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::FCall {
        name: "x".into(),
        args: vec![Expr::Integer(5)],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::IdentifierNotFound>()
        .is_some());
}

#[test]
fn fcall_invalid_param_amount() {
    let mut env = TEnvironment::new();
    env.ftable_set("x".into(), vec![Type::Int], Type::Bool);
    let invalid_identifier = Expr::FCall {
        name: "x".into(),
        args: vec![],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::FCallParametersCountError>()
        .is_some());
}

#[test]
fn fcall_parameters_incompatible() {
    let mut env = TEnvironment::new();
    env.ftable_set("x".into(), vec![Type::Int], Type::Bool);
    let incompatible_parameter = Expr::FCall {
        name: "x".into(),
        args: vec![Expr::Float(5.0)],
    }
    .type_check(&mut env);
    assert!(incompatible_parameter
        .unwrap_err()
        .downcast_ref::<errors::FCallParametersIncompatible>()
        .is_some());
}

#[test]
fn fcall_push_intarray() {
    let mut env = TEnvironment::new();
    let t1 = Expr::FCall {
        name: "push".into(),
        args: vec![Expr::Array(vec![Expr::Integer(5)]),Expr::Integer(5)],
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::IntArray);
}

#[test]
fn fcall_push_empty() {
    let mut env = TEnvironment::new();
    let t1 = Expr::FCall {
        name: "push".into(),
        args: vec![Expr::Array(vec![]),Expr::Integer(5)],
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::IntArray);
}

#[test]
fn fcall_push_typeconflict() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::FCall {
        name: "push".into(),
        args: vec![Expr::Array(vec![Expr::Integer(5)]),Expr::Float(5.0)],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::ErrorInPush>()
        .is_some());
}

#[test]
fn fcall_push_empty_array_with_array() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::FCall {
        name: "push".into(),
        args: vec![Expr::Array(vec![]),Expr::Array(vec![Expr::Integer(5)])],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::ErrorInPush>()
        .is_some());
}


#[test]
fn fcall_push_not_array() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::FCall {
        name: "push".into(),
        args: vec![Expr::Integer(5),Expr::Integer(5)],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::ErrorInPush>()
        .is_some());
}

#[test]
fn fcall_remove_intarray() {
    let mut env = TEnvironment::new();
    let t1 = Expr::FCall {
        name: "remove".into(),
        args: vec![Expr::Array(vec![Expr::Integer(5)]),Expr::Integer(5)],
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::IntArray);
}

#[test]
fn fcall_remove_empty_array() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::FCall {
        name: "remove".into(),
        args: vec![Expr::Array(vec![]),Expr::Integer(5)],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::ErrorInRemove>()
        .is_some());
}

#[test]
fn fcall_remove_not_int_index() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::FCall {
        name: "remove".into(),
        args: vec![Expr::Array(vec![Expr::Integer(5)]),Expr::Float(5.0)],
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::ErrorInRemove>()
        .is_some());
}






#[test]
fn scall() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let t1 = Expr::SCall {
        name: Some("circle".into()),
        args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
        path_poly: None,
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape);
}

#[test]
fn scall_default_path() {
    let mut env = TEnvironment::new();

    let t1 = Expr::SCall {
        name: None,
        args: [("stroke".into(), Expr::Color(
            Expr::Integer(1).into(),
            Expr::Integer(1).into(),
            Expr::Integer(1).into(),
            Expr::Integer(1).into()
        ))].into_iter().collect(),
        path_poly: Some(Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(2).into(), Expr::Integer(3).into()).into(),
            operator: PathOperator::Line
        }.into()),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape);
}

#[test]
fn scall_default_polygon() {
    let mut env = TEnvironment::new();

    let t1 = Expr::SCall {
        name: None,
        args: [("fill".into(), Expr::Color(
            Expr::Integer(1).into(),
            Expr::Integer(1).into(),
            Expr::Integer(1).into(),
            Expr::Integer(1).into()
        ))].into_iter().collect(),
        path_poly : Some(
            Expr::PolygonOperation {
                path: Expr::PathOperation {
                    lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                    rhs: Expr::Point(Expr::Integer(2).into(), Expr::Integer(3).into()).into(),
                    operator: PathOperator::Line
                }.into(),
                operator: PolyOperator::Straight
            }.into()
        )
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape);
}

#[test]
fn scall_invalid_identifier() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::SCall {
        name: Some("circle".into()),
        args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
        path_poly: None,
    }
    .type_check(&mut env);
    assert!(invalid_identifier
        .unwrap_err()
        .downcast_ref::<errors::IdentifierNotFound>()
        .is_some());
}

#[test]
fn scall_parameters_notfound() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let invalid_parameter = Expr::SCall {
        name: Some("circle".into()),
        args: [("circumference".into(), Expr::Float(5.0))]
            .into_iter()
            .collect(),
        path_poly: None,
    }
    .type_check(&mut env);
    assert!(invalid_parameter
        .unwrap_err()
        .downcast_ref::<errors::SCallParameterNotFound>()
        .is_some());
}

#[test]
fn scall_parameters_incompatible() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let incompatible_parameter = Expr::SCall {
        name: Some("circle".into()),
        args: [("radius".into(), Expr::Boolean(true))]
            .into_iter()
            .collect(),
        path_poly: None,
    }
    .type_check(&mut env);
    assert!(incompatible_parameter
        .unwrap_err()
        .downcast_ref::<errors::SCallParametersIncompatible>()
        .is_some());
}

#[test]
fn scall_parameters_incompatible_identifier() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let incompatible_parameter = Expr::SCall {
        name: Some("circle".into()),
        args: [].into_iter().collect(),
        path_poly: None,
    }
    .type_check(&mut env);
    assert!(incompatible_parameter
        .unwrap_err()
        .downcast_ref::<errors::SCallParameterNotFound>()
        .is_some());
}
#[test]
fn member_color() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Color);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "r".to_string(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Int)
}
#[test]
fn member_color_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Color);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "p".to_string(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::MemberAccessColor>()
        .is_some());
}
#[test]
fn member_point() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Point);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "x".to_string(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Float)
}
#[test]
fn member_point_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Point);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "z".to_string(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::MemberAccessPoint>()
        .is_some());
}
#[test]
fn member_shape() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "width".to_string(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Float)
}
#[test]
fn member_shape_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "length".to_string(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::MemberAccessShape>()
        .is_some());
}
#[test]
fn member_array() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::IntArray);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "size".to_string(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Int)
}
#[test]
fn member_array_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::IntArray);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "lenght".to_string(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::MemberAccessArray>()
        .is_some());
}
#[test]
fn member_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Path);
    let t1 = Expr::Member {
        identifier: "x".to_string(),
        member_access: "size".to_string(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::NotAMemberType>()
        .is_some());
}

#[test]
fn scale() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("p".to_string(), Type::Int);
    let t1 = Expr::Scale {
        base_shape: Expr::Variable("x".into()).into(),
        factor: Expr::Variable("p".into()).into(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape)
}
#[test]
fn scale_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("p".to_string(), Type::Point);
    let t1 = Expr::Scale {
        base_shape: Expr::Variable("x".into()).into(),
        factor: Expr::Variable("p".into()).into(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::ManipulationScaleTypeFault>()
        .is_some());
}

#[test]
fn rotate() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("p".to_string(), Type::Int);
    let t1 = Expr::Rotate {
        base_shape: Expr::Variable("x".into()).into(),
        factor: Expr::Variable("p".into()).into(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape)
}

#[test]
fn rotate_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("p".to_string(), Type::Point);
    let t1 = Expr::Rotate {
        base_shape: Expr::Variable("x".into()).into(),
        factor: Expr::Variable("p".into()).into(),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::ManipulationRotateTypeFault>()
        .is_some());
}

#[test]
fn place_with_point() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("y".to_string(), Type::Shape);
    env.vtable_set("p".to_string(), Type::Point);
    let t1 = Expr::Place {
        base_shape: Expr::Variable("x".into()).into(),
        second_shape: Expr::Variable("y".into()).into(),
        point: Some(Expr::Variable("p".into()).into()),
        place_at: "right".to_string(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape)
}

#[test]
fn place_without_point() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("y".to_string(), Type::Shape);
    env.vtable_set("p".to_string(), Type::Point);
    let t1 = Expr::Place {
        base_shape: Expr::Variable("x".into()).into(),
        second_shape: Expr::Variable("y".into()).into(),
        point: None,
        place_at: "right".to_string(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape)
}

#[test]
fn place_invalid() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".to_string(), Type::Shape);
    env.vtable_set("y".to_string(), Type::Int);
    env.vtable_set("p".to_string(), Type::Point);
    let t1 = Expr::Place {
        base_shape: Expr::Variable("x".into()).into(),
        second_shape: Expr::Variable("y".into()).into(),
        place_at: "right".to_string(),
        point: Some(Expr::Variable("p".into()).into()),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::ManipulationPlaceTypeFault>()
        .is_some());
}
