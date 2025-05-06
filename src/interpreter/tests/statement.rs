use crate::{
    interpreter::{environment::IEnvironment, InterpretE, InterpretS},
    program::{
        expression::Expr, operators::binaryoperator::BinaryOperator, r#type::Type, statement::Stmt,
    },
};

#[test]
fn var_decl() {
    let mut env = IEnvironment::new();
    let i1 = Stmt::VarDecl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Expr::Integer(4),
    }
    .interpret(&mut env)
    .unwrap();
    assert_eq!(
        env.vtable_find("x".into()).unwrap().clone(),
        Expr::Integer(4)
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
        Expr::Boolean(true)
    )
}

#[test]
fn fork() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Expr::Integer(4));

    let _ = Stmt::Fork {
        branches: vec![(
            Expr::BinaryOperation {
                lhs: Expr::Integer(1).into(),
                rhs: Expr::Integer(3).into(),
                operator: BinaryOperator::LessThan,
            },
            vec![Stmt::Assign { name: "x".into(), value: Expr::Integer(8) }
            ],
        )],
        otherwise: Option::None,
    }.interpret(&mut env);

    assert_eq!(env.vtable_find("x".into()).unwrap().clone(), Expr::Integer(8))
}

#[test]
fn fork_otherwise() {
    let mut env = IEnvironment::new();

    env.vtable_push("x".into(), Expr::Integer(4));

    let _ = Stmt::Fork {
        branches: vec![(
            Expr::BinaryOperation {
                lhs: Expr::Integer(1).into(),
                rhs: Expr::Integer(3).into(),
                operator: BinaryOperator::GreaterThan,
            },
            vec![Stmt::Assign { name: "x".into(), value: Expr::Integer(8) }
            ],
        )],
        otherwise: Some(vec![Stmt::Assign { name: "x".into(), value: Expr::Integer(9)}]),
    }.interpret(&mut env);

    assert_eq!(env.vtable_find("x".into()).unwrap().clone(), Expr::Integer(9))
}
