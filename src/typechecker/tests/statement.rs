use crate::{
    program::{expression::Expr, r#type::Type, statement::Stmt},
    typechecker::{environment::TEnvironment, errors, TypeCheckS},
};

#[test]
fn vardecl() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::VarDecl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Expr::Integer(5),
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn vardecl_identifier_already_declared() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::Int);
    let already_declared = Stmt::VarDecl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Expr::Integer(5),
    }
    .type_check(&mut env);
    assert!(already_declared
        .unwrap_err()
        .downcast_ref::<errors::IdentifierAlreadyDeclared>()
        .is_some());
}

#[test]
fn vardecl_expression_types_mismatch() {
    let mut env = TEnvironment::new();
    let type_mismatch = Stmt::VarDecl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Expr::Boolean(true),
    }
    .type_check(&mut env);
    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::VariableExpressionTypeNotMatch>()
        .is_some());
}

#[test]
fn funcdecl() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::FuncDecl {
        name: "compare".into(),
        return_type: Type::Bool,
        parameters: vec![("x".into(), Type::Int), ("y".into(), Type::Int)],
        statements: vec![Stmt::VarDecl {
            name: "z".into(),
            declared_type: Type::Int,
            value: Expr::Integer(2),
        }],
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn funcdecl_identifier_already_declared() {
    let mut env = TEnvironment::new();
    env.ftable_set("compare".into(), Vec::new(), Type::Bool);
    let already_declared = Stmt::FuncDecl {
        name: "compare".into(),
        return_type: Type::Bool,
        parameters: Vec::new(),
        statements: Vec::new(),
    }
    .type_check(&mut env);
    assert!(already_declared
        .unwrap_err()
        .downcast_ref::<errors::IdentifierAlreadyDeclared>()
        .is_some());
}

#[test]
fn r#return() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::Return(Expr::Integer(5)).type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn return_mismatch() {
    let mut env = TEnvironment::new();
    env.return_set(Type::Bool);
    let return_mismatch = Stmt::Return(Expr::Integer(5)).type_check(&mut env);
    assert!(return_mismatch
        .unwrap_err()
        .downcast_ref::<errors::ReturnTypeNotMatch>()
        .is_some());
}

#[test]
fn decl() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::Decl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Some(Expr::Integer(5)),
    }
    .type_check(&mut env);

    assert!(t1.is_ok())
}

#[test]
fn decl_optional() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::Decl {
        name: "x".into(),
        declared_type: Type::Int,
        value: None,
    }
    .type_check(&mut env);

    assert!(t1.is_ok())
}

#[test]
fn decl_identifier_already_declared() {
    let mut env = TEnvironment::new();
    env.vdtable_set("x".into(), Type::Int);
    let already_declared = Stmt::Decl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Some(Expr::Integer(5)),
    }
    .type_check(&mut env);

    assert!(already_declared
        .unwrap_err()
        .downcast_ref::<errors::IdentifierAlreadyDeclared>()
        .is_some());
}

#[test]
fn decl_expression_types_mismatch() {
    let mut env = TEnvironment::new();
    let type_mismatch = Stmt::Decl {
        name: "x".into(),
        declared_type: Type::Int,
        value: Some(Expr::Boolean(true)),
    }
    .type_check(&mut env);
    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::VariableExpressionTypeNotMatch>()
        .is_some());
}

#[test]
fn import() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::Import {
        name: "circle".into(),
        path: "./src/typechecker/tests/circle.testfile".into(),
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn import_already_declared() {
    let mut env = TEnvironment::new();
    env.stable_set("circle".into(), [].into_iter().collect());
    let already_declared = Stmt::Import {
        name: "circle".into(),
        path: "./src/typechecker/tests/circle.testfile".into(),
    }
    .type_check(&mut env);
    assert!(already_declared
        .unwrap_err()
        .downcast_ref::<errors::ImportAlreadyDeclared>()
        .is_some());
}

#[test]
fn import_typeerror() {
    let mut env = TEnvironment::new();
    let variable_type_mismatch = Stmt::Import {
        name: "circle".into(),
        path: "./src/typechecker/tests/circle_error.testfile".into(),
    }
    .type_check(&mut env);
    assert!(variable_type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::VariableExpressionTypeNotMatch>()
        .is_some());
}
