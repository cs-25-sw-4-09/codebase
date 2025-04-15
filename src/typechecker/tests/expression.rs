use crate::{program::{expression::Expr, r#type::Type}, typechecker::{environment::TEnvironment, TypeCheckE}};

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