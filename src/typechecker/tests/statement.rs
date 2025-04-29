use crate::{
    program::{expression::Expr, r#type::Type, statement::Stmt},
    typechecker::{
        environment::{EType, TEnvironment},
        errors, TypeCheckS,
    },
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
fn assign() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::Int);
    let t1 = Stmt::Assign {
        name: "x".into(),
        value: Expr::Integer(1),
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn assign_empty() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::ShapeArray);
    let t1 = Stmt::Assign {
        name: "x".into(),
        value: Expr::Array(vec![]),
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn assign_error() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::Float);
    let t1 = Stmt::Assign {
        name: "x".into(),
        value: Expr::Integer(1),
    }
    .type_check(&mut env);
    assert!(t1
        .unwrap_err()
        .downcast_ref::<errors::AssignTypesNoMatch>()
        .is_some());
}

#[test]
fn draw_with_point() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let t1 = Stmt::Draw {
        shape: Expr::SCall {
            name: Some("circle".into()),
            args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
            path_poly: None,
        },
        point: Some(Expr::Point(Expr::Integer(1).into(), Expr::Integer(1).into()).into()),
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn draw_without_point() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let t1 = Stmt::Draw {
        shape: Expr::SCall {
            name: Some("circle".into()),
            args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
            path_poly: None,
        },
        point: None,
    }
    .type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn draw_with_point_shape_error() {
    let mut env = TEnvironment::new();
    let type_mismatch = Stmt::Draw {
        shape: Expr::Integer(1),
        point: Some(Expr::Point(Expr::Integer(1).into(), Expr::Integer(1).into()).into()),
    }
    .type_check(&mut env);

    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::DrawTypeFault>()
        .is_some());
}

#[test]
fn draw_with_point_point_error() {
    let mut env = TEnvironment::new();
    env.stable_set(
        "circle".into(),
        [("radius".into(), EType::DeclNonDefault(Type::Float))]
            .into_iter()
            .collect(),
    );
    let type_mismatch = Stmt::Draw {
        shape: Expr::SCall {
            name: Some("circle".into()),
            args: [("radius".into(), Expr::Float(5.0))].into_iter().collect(),
            path_poly: None,
        },
        point: Some(Expr::Integer(1)),
    }
    .type_check(&mut env);

    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::DrawTypeFault>()
        .is_some());
}

#[test]
fn draw_without_point_shape_error() {
    let mut env = TEnvironment::new();
    let type_mismatch = Stmt::Draw {
        shape: Expr::Integer(1),
        point: None,
    }
    .type_check(&mut env);

    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::DrawTypeFault>()
        .is_some());
}

#[test]
fn for_loop() {
    let mut env = TEnvironment::new();
    let t1 = Stmt::For {
        counter: "x".into(),
        from: Expr::Integer(1),
        to: Expr::Integer(10),
        body: vec![Stmt::VarDecl {
            name: "z".into(),
            declared_type: Type::Int,
            value: Expr::Integer(2),
        }],
    }.type_check(&mut env);
    assert!(t1.is_ok())
}

#[test]
fn for_loop_counter_error() {
    let mut env = TEnvironment::new();
    env.vtable_set("x".into(), Type::Int);
    let type_mismatch = Stmt::For {
        counter: "x".into(),
        from: Expr::Integer(1),
        to: Expr::Integer(10),
        body: vec![Stmt::VarDecl {
            name: "z".into(),
            declared_type: Type::Int,
            value: Expr::Integer(2),
        }],
    }.type_check(&mut env);
    
    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::ForLoopCounterDeclared>()
        .is_some());
}

#[test]
fn for_loop_range_error() {
    let mut env = TEnvironment::new();
    let type_mismatch = Stmt::For {
        counter: "x".into(),
        from: Expr::Float(1.2),
        to: Expr::Integer(10),
        body: vec![Stmt::VarDecl {
            name: "z".into(),
            declared_type: Type::Int,
            value: Expr::Integer(2),
        }],
    }.type_check(&mut env);
    
    assert!(type_mismatch
        .unwrap_err()
        .downcast_ref::<errors::ForLoopTypeError>()
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
    env.vdtable_set_non_default("x".into(), Type::Int);
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
