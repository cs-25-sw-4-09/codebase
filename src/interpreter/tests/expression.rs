use crate::{
    interpreter::{environment::IEnvironment, errors, InterpretE},
    program::{
        expression::Expr,
        operators::{binaryoperator::BinaryOperator, unaryoperator::UnaryOperator},
    },
};

#[test]
fn integer() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Integer(4).interpret(&mut env).unwrap();
    assert_eq!(i1, Expr::Integer(4))
}

#[test]
fn float() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Float(4.8).interpret(&mut env).unwrap();
    assert_eq!(i1, Expr::Float(4.8))
}

#[test]
fn boolean() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Boolean(true).interpret(&mut env).unwrap();
    assert_eq!(i1, Expr::Boolean(true))
}

#[test]
fn color() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Color(
        Expr::Integer(32).into(),
        Expr::Integer(82).into(),
        Expr::Integer(115).into(),
        Expr::Integer(255).into(),
    )
    .interpret(&mut env)
    .unwrap();
    assert_eq!(
        i1,
        Expr::Color(
            Expr::Integer(32).into(),
            Expr::Integer(82).into(),
            Expr::Integer(115).into(),
            Expr::Integer(255).into()
        )
    )
}

#[test]
fn variable() {
    let mut env = IEnvironment::new();
    env.vtable_push("x".into(), Expr::Integer(4));

    let i1 = Expr::Variable("x".into()).interpret(&mut env).unwrap();
    assert_eq!(i1, Expr::Integer(4))
}

#[test]
fn addition() {
    let mut env = IEnvironment::new();
    let i1 = Expr::BinaryOperation {
        lhs: Expr::Integer(4).into(),
        rhs: Expr::Integer(7).into(),
        operator: BinaryOperator::Add,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::BinaryOperation {
        lhs: Expr::Float(4.8).into(),
        rhs: Expr::Integer(7).into(),
        operator: BinaryOperator::Add,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Expr::Integer(11));
    assert_eq!(i2, Expr::Float(11.8));
}

#[test]
fn subtract() {
    let mut env = IEnvironment::new();
    let i1 = Expr::BinaryOperation {
        lhs: Expr::Integer(4).into(),
        rhs: Expr::Integer(7).into(),
        operator: BinaryOperator::Subtract,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::BinaryOperation {
        lhs: Expr::Float(4.8).into(),
        rhs: Expr::Integer(7).into(),
        operator: BinaryOperator::Subtract,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Expr::Integer(-3));
    assert_eq!(i2, Expr::Float(-2.2));
}

#[test]
fn multiply() {
    let mut env = IEnvironment::new();
    let i1 = Expr::BinaryOperation {
        lhs: Expr::Integer(2).into(),
        rhs: Expr::Integer(8).into(),
        operator: BinaryOperator::Multiply,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::BinaryOperation {
        lhs: Expr::Float(3.5).into(),
        rhs: Expr::Integer(2).into(),
        operator: BinaryOperator::Multiply,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Expr::Integer(16));
    assert_eq!(i2, Expr::Float(7.0));
}

#[test]
fn divide() {
    let mut env = IEnvironment::new();
    let i1 = Expr::BinaryOperation {
        lhs: Expr::Integer(10).into(),
        rhs: Expr::Integer(2).into(),
        operator: BinaryOperator::Divide,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::BinaryOperation {
        lhs: Expr::Float(7.0).into(),
        rhs: Expr::Integer(7).into(),
        operator: BinaryOperator::Divide,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Expr::Integer(5));
    assert_eq!(i2, Expr::Float(1.0));
}

#[test]
fn divide_by_0() {
    let mut env = IEnvironment::new();
    let i1 = Expr::BinaryOperation {
        lhs: Expr::Integer(10).into(),
        rhs: Expr::Integer(0).into(),
        operator: BinaryOperator::Divide,
    }
    .interpret(&mut env);

    assert!(i1
        .unwrap_err()
        .downcast_ref::<errors::DivideByZero>()
        .is_some());
}

#[test]
fn modulus() {
    let mut env = IEnvironment::new();
    let i1 = Expr::BinaryOperation {
        lhs: Expr::Integer(10).into(),
        rhs: Expr::Integer(3).into(),
        operator: BinaryOperator::Modulus,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::BinaryOperation {
        lhs: Expr::Float(3.5).into(),
        rhs: Expr::Integer(1).into(),
        operator: BinaryOperator::Modulus,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Expr::Integer(1));
    assert_eq!(i2, Expr::Float(0.5));
}

#[test]
fn negate() {
    let mut env = IEnvironment::new();
    let i1 = Expr::UnaryOperation {
        operator: UnaryOperator::Negate,
        expr: Expr::Boolean(true).into(),
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Expr::Boolean(false));
}
