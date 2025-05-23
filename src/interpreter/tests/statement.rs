use crate::{
    interpreter::{
        data_types::line::Line,
        environment::IEnvironment,
        errors,
        value::Value,
        InterpretE, InterpretS,
    },
    program::{
        expression::Expr,
        operators::{binaryoperator::BinaryOperator, pathoperator::PathOperator},
        r#type::Type,
        statement::Stmt,
    },
};

#[test]
fn var_decl() {
    let mut env = IEnvironment::new();
    let _ = Stmt::VarDecl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Expr::Integer(4),
    }
    .interpret(&mut env);
    assert_eq!(
        env.vtable_find("x".into()).unwrap().clone(),
        Value::Integer(4)
    )
}

#[test]
fn func_decl_no_return() {
    let mut env = IEnvironment::new();
    let _ = Stmt::FuncDecl {
        name: "f".into(),
        return_type: Type::Bool,
        parameters: vec![("x".into(), Type::Int)],
        statements: vec![Stmt::VarDecl {
            name: "y".into(),
            declared_type: Type::Int,
            value: Expr::Integer(4),
        }],
    }
    .interpret(&mut env);
    let i1 = Expr::FCall {
        name: "f".into(),
        args: vec![Expr::Integer(4)],
    }
    .interpret(&mut env);
    assert!(i1
        .unwrap_err()
        .downcast_ref::<errors::FunctionNotReturning>()
        .is_some()
    )
}

#[test]
fn func_decl_return() {
    let mut env = IEnvironment::new();
    let _ = Stmt::FuncDecl {
        name: "f".into(),
        return_type: Type::Bool,
        parameters: vec![("x".into(), Type::Int)],
        statements: vec![Stmt::Return(Expr::Boolean(true))],
    }
    .interpret(&mut env);
    assert_eq!(
        Expr::FCall {
            name: "f".into(),
            args: vec![Expr::Integer(4)]
        }
        .interpret(&mut env)
        .unwrap(),
        Value::Boolean(true)
    )
}

#[test]
fn fork() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Value::Integer(4));

    let _ = Stmt::Fork {
        branches: vec![(
            Expr::BinaryOperation {
                lhs: Expr::Integer(1).into(),
                rhs: Expr::Integer(3).into(),
                operator: BinaryOperator::LessThan,
            },
            vec![Stmt::Assign {
                name: "x".into(),
                value: Expr::Integer(8),
            }],
        )],
        otherwise: Option::None,
    }
    .interpret(&mut env);

    assert_eq!(
        env.vtable_find("x".into()).unwrap().clone(),
        Value::Integer(8)
    )
}

#[test]
fn fork_otherwise() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Value::Integer(4));

    let _ = Stmt::Fork {
        branches: vec![(
            Expr::BinaryOperation {
                lhs: Expr::Integer(1).into(),
                rhs: Expr::Integer(3).into(),
                operator: BinaryOperator::GreaterThan,
            },
            vec![Stmt::Assign {
                name: "x".into(),
                value: Expr::Integer(8),
            }],
        )],
        otherwise: Some(vec![Stmt::Assign {
            name: "x".into(),
            value: Expr::Integer(9),
        }]),
    }
    .interpret(&mut env);

    assert_eq!(
        env.vtable_find("x".into()).unwrap().clone(),
        Value::Integer(9)
    )
}

#[test]
fn draw_without_place() {
    let mut env = IEnvironment::new();

    Stmt::Draw {
        shape: Expr::SCall {
            name: None,
            args: vec![(
                "stroke".to_owned(),
                Expr::Color(
                    Expr::Integer(255).into(),
                    Expr::Integer(255).into(),
                    Expr::Integer(255).into(),
                    Expr::Integer(255).into(),
                )
                .into(),
            )]
            .into_iter()
            .collect(),
            path_poly: Some(
                Expr::PathOperation {
                    lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                    rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
                    operator: PathOperator::Line,
                }
                .into(),
            ),
        },
        point: None,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        Value::Shape(env.darray_get().clone()),
        Value::Shape(
            vec![(
                vec![Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ])]
                .into(),
                vec![(
                    "stroke".to_owned(),
                    Value::Color(
                        Value::Integer(255).into(),
                        Value::Integer(255).into(),
                        Value::Integer(255).into(),
                        Value::Integer(255).into()
                    )
                    .into()
                )]
                .into_iter()
                .collect()
            )
                .into()]
            .into(),
        )
    )
}

#[test]
fn draw_with_place() {
    let mut env = IEnvironment::new();

    Stmt::Draw {
        shape: Expr::SCall {
            name: None,
            args: vec![(
                "stroke".to_owned(),
                Expr::Color(
                    Expr::Integer(255).into(),
                    Expr::Integer(255).into(),
                    Expr::Integer(255).into(),
                    Expr::Integer(255).into(),
                )
                .into(),
            )]
            .into_iter()
            .collect(),
            path_poly: Some(
                Expr::PathOperation {
                    lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                    rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
                    operator: PathOperator::Line,
                }
                .into(),
            ),
        },
        point: Some(Expr::Point(Expr::Integer(4).into(), Expr::Integer(5).into()).into()),
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        Value::Shape(env.darray_get().clone()),
        Value::Shape(
            vec![(
                vec![Line::Straight(vec![
                    (Value::Integer(4), Value::Integer(3)).into(),
                    (Value::Integer(6), Value::Integer(5)).into()
                ])]
                .into(),
                vec![(
                    "stroke".to_owned(),
                    Value::Color(
                        Value::Integer(255).into(),
                        Value::Integer(255).into(),
                        Value::Integer(255).into(),
                        Value::Integer(255).into()
                    )
                    .into()
                )]
                .into_iter()
                .collect()
            )
                .into()]
            .into(),
        )
    )
}


#[test]
fn import() {
    let mut env = IEnvironment::new();
    Stmt::Import {
        name: "test".into(),
        path: "./src/interpreter/tests/test_imports/dummyfile.extension".into(),
    }
    .interpret(&mut env)
    .unwrap();

    assert!(env.stable_find("test".into()).is_some())
}

#[test]
fn for_loop() {
    let mut env = IEnvironment::new();
    env.vtable_push("count".into(), Value::Integer(0));
    Stmt::For {
        counter: "i".into(),
        from: Expr::Integer(0),
        to: Expr::Integer(5),
        body: vec![Stmt::Assign {
            name: "count".into(),
            value: Expr::BinaryOperation {
                lhs: Expr::Variable("count".into()).into(),
                rhs: Expr::Integer(1).into(),
                operator: BinaryOperator::Add,
            }
            .into(),
        }],
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        env.vtable_find("count".into()).unwrap().clone(),
        Value::Integer(5)
    )
}

#[test]
fn for_loop_return() {
    let mut env = IEnvironment::new();
    env.vtable_push("count".into(), Value::Integer(0));
    Stmt::For {
        counter: "i".into(),
        from: Expr::Integer(0),
        to: Expr::Integer(5),
        body: vec![
            Stmt::Assign {
                name: "count".into(),
                value: Expr::BinaryOperation {
                    lhs: Expr::Variable("count".into()).into(),
                    rhs: Expr::Integer(1).into(),
                    operator: BinaryOperator::Add,
                }
                .into(),
            },
            Stmt::Return(Expr::Integer(0)),
        ],
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        env.vtable_find("count".into()).unwrap().clone(),
        Value::Integer(1)
    )
}

#[test]
fn decl_non_default() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Value::Integer(4));

    let i1 = Stmt::Decl {
        name: "x".into(),
        declared_type: Type::Int,
        value: None,
    }
    .interpret(&mut env);
    let i2 = Stmt::Decl {
        name: "y".into(),
        declared_type: Type::Int,
        value: None,
    }
    .interpret(&mut env);

    assert!(i1.is_ok());

    assert!(i2
        .unwrap_err()
        .downcast_ref::<errors::DeclValueNotSpecified>()
        .is_some())
}

#[test]
fn decl_default() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Value::Integer(4));

    let i1 = Stmt::Decl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Some(Expr::Integer(3)),
    }
    .interpret(&mut env);
    Stmt::Decl {
        name: "y".into(),
        declared_type: Type::Int,
        value: Some(Expr::Integer(3)),
    }
    .interpret(&mut env).unwrap();

    assert!(i1.is_ok());

    assert_eq!(env.vtable_find("y".into()).unwrap().clone(), Value::Integer(3))
}

#[test]
fn array_assign() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]));

    let i1 = Stmt::ArrayAssign{ name: "x".into(), value: Expr::Integer(5), index: Expr::Integer(1)}
    .interpret(&mut env);

    assert!(i1.is_ok());

    assert_eq!(env.vtable_find("x".into()).unwrap().clone(),Value::Array(vec![Value::Integer(1), Value::Integer(5), Value::Integer(3)]))
}
