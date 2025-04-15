use crate::{
    program::{
        expression::Expr,
        operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator},
        r#type::Type,
    },
    typechecker::{environment::TEnvironment, errors, TypeCheckE},
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
fn variable() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::Int);
    let t1 = Expr::Variable("x".into()).type_check(&mut env).unwrap();
    assert_eq!(t1, Type::Int)
}

#[test]
fn binary_operation_arithmetics() {
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
fn binary_operation_comparisons() {
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
fn scall() {
    let mut env = TEnvironment::new();
    env.stable_set("circle".into(), [("radius".into(), Type::Float)].into_iter().collect());
    let t1 = Expr::SCall {
        name: "circle".into(),
        args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
    }
    .type_check(&mut env)
    .unwrap();
    assert_eq!(t1, Type::Shape);
}

