use crate::{
    interpreter::{environment::IEnvironment, InterpretE, InterpretS},
    program::{expression::Expr, r#type::Type, statement::Stmt},
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


