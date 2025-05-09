use crate::{
    interpreter::{
        data_types::{
            figure::{Figure, Line},
            point::Point,
        },
        environment::IEnvironment,
        errors,
        value::Value,
        InterpretE, InterpretS,
    },
    program::{
        expression::Expr,
        operators::{
            binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
            unaryoperator::UnaryOperator,
        },
        r#type::Type,
        statement::Stmt,
    },
};

#[test]
fn integer() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Integer(4).interpret(&mut env).unwrap();
    assert_eq!(i1, Value::Integer(4))
}

#[test]
fn float() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Float(4.8).interpret(&mut env).unwrap();
    assert_eq!(i1, Value::Float(4.8))
}

#[test]
fn boolean() {
    let mut env = IEnvironment::new();
    let i1 = Expr::Boolean(true).interpret(&mut env).unwrap();
    assert_eq!(i1, Value::Boolean(true))
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
        Value::Color(
            Value::Integer(32).into(),
            Value::Integer(82).into(),
            Value::Integer(115).into(),
            Value::Integer(255).into()
        )
    )
}

#[test]
fn variable() {
    let mut env = IEnvironment::new();
    env.vtable_push("x".into(), Value::Integer(4));

    let i1 = Expr::Variable("x".into()).interpret(&mut env).unwrap();
    assert_eq!(i1, Value::Integer(4))
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

    let i3 = Expr::BinaryOperation {
        lhs: Expr::Float(4.0).into(),
        rhs: Expr::Float(7.0).into(),
        operator: BinaryOperator::Add,
    }
    .interpret(&mut env)
    .unwrap();

    let i4 = Expr::BinaryOperation {
        lhs: Expr::Integer(4).into(),
        rhs: Expr::Float(7.5).into(),
        operator: BinaryOperator::Add,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Value::Integer(11));
    assert_eq!(i2, Value::Float(11.8));
    assert_eq!(i3, Value::Float(11.0));
    assert_eq!(i4, Value::Float(11.5));
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

    let i3 = Expr::BinaryOperation {
        lhs: Expr::Float(4.0).into(),
        rhs: Expr::Float(7.0).into(),
        operator: BinaryOperator::Subtract,
    }
    .interpret(&mut env)
    .unwrap();

    let i4 = Expr::BinaryOperation {
        lhs: Expr::Integer(4).into(),
        rhs: Expr::Float(7.5).into(),
        operator: BinaryOperator::Subtract,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i3, Value::Float(-3.0));
    assert_eq!(i4, Value::Float(-3.5));
    assert_eq!(i1, Value::Integer(-3));
    assert_eq!(i2, Value::Float(-2.2));
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

    let i3 = Expr::BinaryOperation {
        lhs: Expr::Float(4.0).into(),
        rhs: Expr::Float(7.0).into(),
        operator: BinaryOperator::Multiply,
    }
    .interpret(&mut env)
    .unwrap();

    let i4 = Expr::BinaryOperation {
        lhs: Expr::Integer(4).into(),
        rhs: Expr::Float(2.5).into(),
        operator: BinaryOperator::Multiply,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i3, Value::Float(28.0));
    assert_eq!(i4, Value::Float(10.0));
    assert_eq!(i1, Value::Integer(16));
    assert_eq!(i2, Value::Float(7.0));
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

    let i3 = Expr::BinaryOperation {
        lhs: Expr::Float(8.0).into(),
        rhs: Expr::Float(2.0).into(),
        operator: BinaryOperator::Divide,
    }
    .interpret(&mut env)
    .unwrap();

    let i4 = Expr::BinaryOperation {
        lhs: Expr::Integer(16).into(),
        rhs: Expr::Float(4.5).into(),
        operator: BinaryOperator::Divide,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i3, Value::Float(4.0));
    assert_eq!(i4, Value::Float(3.5555555555555554));
    assert_eq!(i1, Value::Integer(5));
    assert_eq!(i2, Value::Float(1.0));
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

    let i3 = Expr::BinaryOperation {
        lhs: Expr::Float(5.4).into(),
        rhs: Expr::Float(2.2).into(),
        operator: BinaryOperator::Modulus,
    }
    .interpret(&mut env)
    .unwrap();

    let i4 = Expr::BinaryOperation {
        lhs: Expr::Integer(16).into(),
        rhs: Expr::Float(4.5).into(),
        operator: BinaryOperator::Modulus,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i3, Value::Float(1.0));
    assert_eq!(i4, Value::Float(2.5));

    assert_eq!(i1, Value::Integer(1));
    assert_eq!(i2, Value::Float(0.5));
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

    assert_eq!(i1, Value::Boolean(false));
}

#[test]
fn negative() {
    //Integer
    let mut env = IEnvironment::new();
    let i1 = Expr::UnaryOperation {
        operator: UnaryOperator::Negative,
        expr: Expr::Integer(4).into(),
    }
    .interpret(&mut env)
    .unwrap();

    //Float
    let i2 = Expr::UnaryOperation {
        operator: UnaryOperator::Negative,
        expr: Expr::Float(2.0).into(),
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i1, Value::Integer(-4));
    assert_eq!(i2, Value::Float(-2.0));
}

#[test]
fn member_color() {
    let mut env = IEnvironment::new();

    let _ = Stmt::VarDecl {
        name: "color".into(),
        declared_type: Type::Color,
        value: Expr::Color(
            Box::new(Expr::Integer(4)),
            Box::new(Expr::Integer(5)),
            Box::new(Expr::Integer(6)),
            Box::new(Expr::Integer(7)),
        ),
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::Member {
        identifier: "color".into(),
        member_access: "r".into(),
    }
    .interpret(&mut env)
    .unwrap();
    let i3 = Expr::Member {
        identifier: "color".into(),
        member_access: "g".into(),
    }
    .interpret(&mut env)
    .unwrap();
    let i4 = Expr::Member {
        identifier: "color".into(),
        member_access: "b".into(),
    }
    .interpret(&mut env)
    .unwrap();
    let i5 = Expr::Member {
        identifier: "color".into(),
        member_access: "a".into(),
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i2, Value::Integer(4));
    assert_eq!(i3, Value::Integer(5));
    assert_eq!(i4, Value::Integer(6));
    assert_eq!(i5, Value::Integer(7));
}

#[test]
fn member_point() {
    let mut env = IEnvironment::new();

    let _ = Stmt::VarDecl {
        name: "point".into(),
        declared_type: Type::Point,
        value: Expr::Point(Box::new(Expr::Integer(4)), Box::new(Expr::Integer(5))),
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::Member {
        identifier: "point".into(),
        member_access: "x".into(),
    }
    .interpret(&mut env)
    .unwrap();
    let i3 = Expr::Member {
        identifier: "point".into(),
        member_access: "y".into(),
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(i2, Value::Integer(4));
    assert_eq!(i3, Value::Integer(5));
}

/*#[test]
fn member_shape() {
    let mut env = IEnvironment::new();

    let _ = Stmt::VarDecl {
        name: "shape".into(),
        declared_type: Type::Shape,
        value: Expr::Shape(Box::new(Expr::Integer(4)), Box::new(Expr::Integer(5)))
    }.interpret(&mut env).unwrap();

    let i2 = Expr::Member { identifier: "point".into(), member_access: "x".into() }.interpret(&mut env).unwrap();
    let i3 = Expr::Member { identifier: "point".into(), member_access: "y".into() }.interpret(&mut env).unwrap();


    assert_eq!(i2, Value::Integer(4));
    assert_eq!(i3, Value::Integer(5));
}*/

#[test]
fn array_size() {
    let mut env = IEnvironment::new();
    let _ = Stmt::VarDecl {
        name: "array".into(),
        declared_type: Type::IntArray,
        value: Expr::Array(vec![Expr::Integer(4), Expr::Integer(5)]),
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::Member {
        identifier: "array".into(),
        member_access: "size".into(),
    }
    .interpret(&mut env)
    .unwrap();
    assert_eq!(i2, Value::Integer(2));
}

#[test]
fn array_interpret() {
    let mut env = IEnvironment::new();

    let i2 = Expr::Array(vec![Expr::Integer(4)])
        .interpret(&mut env)
        .unwrap();

    assert_eq!(i2, Value::Array(vec![Box::new(Value::Integer(4))]));
}

#[test]
fn pathoperation_point_point() {
    let mut env = IEnvironment::new();

    let i1 = Expr::PathOperation {
        lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
        rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
        operator: PathOperator::Line,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::PathOperation {
        lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
        rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        i1,
        Value::Figure(
            vec![Line::Straight(vec![
                (Value::Integer(1), Value::Integer(2)).into(),
                (Value::Integer(3), Value::Integer(4)).into()
            ])]
            .into()
        )
    );
    assert_eq!(
        i2,
        Value::Figure(
            vec![Line::Curved(vec![
                (Value::Integer(1), Value::Integer(2)).into(),
                (Value::Integer(3), Value::Integer(4)).into()
            ])]
            .into()
        )
    );
}

#[test]
fn pathoperation_path_point() {
    let mut env = IEnvironment::new();

    let i1 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        rhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
        operator: PathOperator::Line,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        rhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    let i3 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        rhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        i1,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ])
            ]
            .into()
        )
    );
    assert_eq!(
        i2,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Curved(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i3,
        Value::Figure(
            vec![Line::Curved(vec![
                (Value::Integer(1), Value::Integer(2)).into(),
                (Value::Integer(3), Value::Integer(4)).into(),
                (Value::Integer(5), Value::Integer(6)).into()
            ])]
            .into()
        )
    );
}

#[test]
fn pathoperation_point_path() {
    let mut env = IEnvironment::new();

    let i1 = Expr::PathOperation {
        lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            rhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PathOperator::Line,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::PathOperation {
        lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            rhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    let i3 = Expr::PathOperation {
        lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            rhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        i1,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ])
            ]
            .into()
        )
    );
    assert_eq!(
        i2,
        Value::Figure(
            vec![
                Line::Curved(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i3,
        Value::Figure(
            vec![Line::Curved(vec![
                (Value::Integer(1), Value::Integer(2)).into(),
                (Value::Integer(3), Value::Integer(4)).into(),
                (Value::Integer(5), Value::Integer(6)).into()
            ])]
            .into()
        )
    );
}

#[test]
fn pathoperation_path_path() {
    let mut env = IEnvironment::new();

    let i1 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PathOperator::Line,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    let i3 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    let i4 = Expr::PathOperation {
        lhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        rhs: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
            rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PathOperator::Curve,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        i1,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(5), Value::Integer(6)).into(),
                    (Value::Integer(7), Value::Integer(8)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i2,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Curved(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into(),
                    (Value::Integer(7), Value::Integer(8)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i4,
        Value::Figure(
            vec![
                Line::Curved(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(5), Value::Integer(6)).into(),
                    (Value::Integer(7), Value::Integer(8)).into()
                ])
            ]
            .into()
        )
    );
}

#[test]
fn polygonoperation_straight() {
    let mut env = IEnvironment::new();

    let i1 = Expr::PolygonOperation {
        path: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PolyOperator::Straight,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::PolygonOperation {
        path: Expr::PathOperation {
            lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
            rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
            operator: PathOperator::Curve,
        }
        .into(),
        operator: PolyOperator::Straight,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        i1,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(1), Value::Integer(2)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i2,
        Value::Figure(
            vec![
                Line::Curved(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(1), Value::Integer(2)).into()
                ])
            ]
            .into()
        )
    );
}

#[test]
fn polygonoperation_curve() {
    let mut env = IEnvironment::new();

    let i1 = Expr::PolygonOperation {
        path: Expr::PathOperation {
            lhs: Expr::PathOperation {
                lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
                operator: PathOperator::Line,
            }
            .into(),
            rhs: Expr::PathOperation {
                lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
                rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
                operator: PathOperator::Curve,
            }
            .into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PolyOperator::Curved,
    }
    .interpret(&mut env)
    .unwrap();

    let i2 = Expr::PolygonOperation {
        path: Expr::PathOperation {
            lhs: Expr::PathOperation {
                lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
                operator: PathOperator::Curve,
            }
            .into(),
            rhs: Expr::PathOperation {
                lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
                rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
                operator: PathOperator::Line,
            }
            .into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PolyOperator::Curved,
    }
    .interpret(&mut env)
    .unwrap();

    let i3 = Expr::PolygonOperation {
        path: Expr::PathOperation {
            lhs: Expr::PathOperation {
                lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                rhs: Expr::Point(Expr::Integer(3).into(), Expr::Integer(4).into()).into(),
                operator: PathOperator::Curve,
            }
            .into(),
            rhs: Expr::PathOperation {
                lhs: Expr::Point(Expr::Integer(5).into(), Expr::Integer(6).into()).into(),
                rhs: Expr::Point(Expr::Integer(7).into(), Expr::Integer(8).into()).into(),
                operator: PathOperator::Curve,
            }
            .into(),
            operator: PathOperator::Line,
        }
        .into(),
        operator: PolyOperator::Curved,
    }
    .interpret(&mut env)
    .unwrap();

    assert_eq!(
        i1,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ]),
                Line::Curved(vec![
                    (Value::Integer(5), Value::Integer(6)).into(),
                    (Value::Integer(7), Value::Integer(8)).into(),
                    (Value::Integer(1), Value::Integer(2)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i2,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(5), Value::Integer(6)).into(),
                    (Value::Integer(7), Value::Integer(8)).into()
                ]),
                Line::Curved(vec![
                    (Value::Integer(7), Value::Integer(8)).into(),
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ])
            ]
            .into()
        )
    );

    assert_eq!(
        i3,
        Value::Figure(
            vec![
                Line::Straight(vec![
                    (Value::Integer(3), Value::Integer(4)).into(),
                    (Value::Integer(5), Value::Integer(6)).into()
                ]),
                Line::Curved(vec![
                    (Value::Integer(5), Value::Integer(6)).into(),
                    (Value::Integer(7), Value::Integer(8)).into(),
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(3), Value::Integer(4)).into()
                ])
            ]
            .into()
        )
    );
}
