use crate::{
    program::{
        expression::Expr,
        operators::{
            binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
            unaryoperator::UnaryOperator,
        },
        r#type::Type,
    },
    typechecker::{environment::{EType, TEnvironment}, errors, TypeCheckE},
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
        operator: PolyOperator::Polygon,
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
        operator: PolyOperator::Polygon,
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
fn scall() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))].into_iter().collect(),
    );
    let t1 = Expr::SCall {
        name: "circle".into(),
        args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape);
}

#[test]
fn scall_invalid_identifier() {
    let mut env = TEnvironment::new();
    let invalid_identifier = Expr::SCall {
        name: "circle".into(),
        args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
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
        [("radius".into(), EType::DeclNonDefault(Type::Float))].into_iter().collect(),
    );
    let invalid_parameter = Expr::SCall {
        name: "circle".into(),
        args: [("circumference".into(), Expr::Float(5.0))]
            .into_iter()
            .collect(),
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
        [("radius".into(), EType::DeclNonDefault(Type::Float))].into_iter().collect(),
    );
    let incompatible_parameter = Expr::SCall {
        name: "circle".into(),
        args: [("radius".into(), Expr::Boolean(true))]
            .into_iter()
            .collect(),
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
        [("radius".into(), EType::DeclNonDefault(Type::Float))].into_iter().collect(),
    );
    let incompatible_parameter = Expr::SCall {
        name: "circle".into(),
        args: []
            .into_iter()
            .collect(),
    }
    .type_check(&mut env);
    assert!(incompatible_parameter
        .unwrap_err()
        .downcast_ref::<errors::SCallParameterNotFound>()
        .is_some());
}
