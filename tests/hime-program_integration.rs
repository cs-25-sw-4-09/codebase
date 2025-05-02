use std::{collections::HashMap, vec};

use codebase::{
    program::{
        expression::Expr,
        operators::{
            binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
            unaryoperator::UnaryOperator,
        },
        program,
        r#type::Type,
        statement::Stmt,
    },
    typechecker::TypeCheckP,
};

//----------------------------------------------------
//Tests for Declaration of types in construction field
//----------------------------------------------------

#[test]
fn test_program_new_converts_ast_to_program_fcall() {
    let code = "begin
    x: int = f(1);";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::FCall {
                name: "f".to_string(),
                args: vec![Expr::Integer(1)]
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_int() {
    let code = "begin
    x: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(value, &Expr::Integer(1));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_color() {
    let code = "begin
    x: color = (0,0,0,0);";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Color);
        assert_eq!(
            value,
            &Expr::Color(
                Box::new(Expr::Integer(0)),
                Box::new(Expr::Integer(0)),
                Box::new(Expr::Integer(0)),
                Box::new(Expr::Integer(0))
            )
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_point() {
    let code = "begin
    x: point = (0,0);";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Point);
        assert_eq!(
            value,
            &Expr::Point(Box::new(Expr::Integer(0)), Box::new(Expr::Integer(0)))
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_float() {
    let code = "begin
    x: float = 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(value, &Expr::Float(1.0));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_bool() {
    let code = "begin
    x: bool = true;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(value, &Expr::Boolean(true));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_scall_with_params() {
    let code = "begin
    myShape: shape = dummy(|x = 1, b = true|);";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "myShape");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::SCall {
                name: Some("dummy".into()),
                args: HashMap::from([
                    ("x".to_string(), Expr::Integer(1)),
                    ("b".to_string(), Expr::Boolean(true))
                ]),
                path_poly: None
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_scall_path_with_params() {
    let code = "begin
    myShape: shape = (1,2)--(2,3)(|fill = (1,1,1,1)|);";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "myShape");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::SCall {
                name: None,
                args: HashMap::from([(
                    "fill".to_string(),
                    Expr::Color(
                        Expr::Integer(1).into(),
                        Expr::Integer(1).into(),
                        Expr::Integer(1).into(),
                        Expr::Integer(1).into()
                    )
                ),]),
                path_poly: Some(
                    Expr::PathOperation {
                        lhs: Expr::Point(Expr::Integer(1).into(), Expr::Integer(2).into()).into(),
                        rhs: Expr::Point(Expr::Integer(2).into(), Expr::Integer(3).into()).into(),
                        operator: PathOperator::Line
                    }
                    .into()
                )
            }
        );
    }
}
#[test]
fn test_program_new_converts_ast_to_program_scall_polygon_with_params() {
    let code = "begin
    myShape: shape = (1,2)--*(|fill = (1,1,1,1)|);";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "myShape");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::SCall {
                name: None,
                args: HashMap::from([(
                    "fill".to_string(),
                    Expr::Color(
                        Expr::Integer(1).into(),
                        Expr::Integer(1).into(),
                        Expr::Integer(1).into(),
                        Expr::Integer(1).into()
                    )
                ),]),
                path_poly: Some(
                    Expr::PolygonOperation {
                        path: Box::new(Expr::Point(
                            Box::new(Expr::Integer(1)),
                            Box::new(Expr::Integer(2))
                        )),
                        operator: PolyOperator::Polygon
                    }
                    .into()
                )
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_scall_without_params() {
    let code = "begin
    myShape: shape = dummy(||);";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "myShape");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::SCall {
                name: Some("dummy".into()),
                args: HashMap::from([]),
                path_poly: None
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_path_straight() {
    let code = "begin
    x: path = (1,2)--(3,4);";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Path);
        assert_eq!(
            value,
            &Expr::PathOperation {
                lhs: Box::new(Expr::Point(
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2))
                )),
                rhs: Box::new(Expr::Point(
                    Box::new(Expr::Integer(3)),
                    Box::new(Expr::Integer(4))
                )),
                operator: PathOperator::Line
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_path_curved() {
    let code = "begin
    x: path = (1,2)~~(3,4);";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Path);
        assert_eq!(
            value,
            &Expr::PathOperation {
                lhs: Box::new(Expr::Point(
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2))
                )),
                rhs: Box::new(Expr::Point(
                    Box::new(Expr::Integer(3)),
                    Box::new(Expr::Integer(4))
                )),
                operator: PathOperator::Curve
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_polygon_straight() {
    let code = "begin
    x: polygon = (1,2)--*;";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Polygon);
        assert_eq!(
            value,
            &Expr::PolygonOperation {
                path: Box::new(Expr::Point(
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2))
                )),
                operator: PolyOperator::Polygon
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_polygon_curve() {
    let code = "begin
    x: polygon = (1,2)~~*;";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Polygon);
        assert_eq!(
            value,
            &Expr::PolygonOperation {
                path: Box::new(Expr::Point(
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2))
                )),
                operator: PolyOperator::Polygon
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_array_empty() {
    let code = "begin
    x: int[] = [];";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::IntArray);
        assert_eq!(value, &Expr::Array(vec![]));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_array_not_empty() {
    let code = "begin
    x: int[] = [1];";

    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::IntArray);
        assert_eq!(value, &Expr::Array(vec![Expr::Integer(1)]));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_member_access_color() {
    let code = "begin
    y:int = x.r;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "y");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::Member {
                identifier: "x".to_string(),
                member_access: "r".to_string()
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_member_access_shape() {
    let code = "begin
    y:shape = x.width;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "y");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::Member {
                identifier: "x".to_string(),
                member_access: "width".to_string()
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_member_access_point() {
    let code = "begin
    y:float = x.x;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "y");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::Member {
                identifier: "x".to_string(),
                member_access: "x".to_string()
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_member_access_array() {
    let code = "begin
    y:int = x.size;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "y");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::Member {
                identifier: "x".to_string(),
                member_access: "size".to_string()
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_place() {
    let code = "begin
    z:shape = place x ontop y by (1,2);
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "z");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::Place {
                base_shape: Expr::Variable("x".to_string()).into(),
                second_shape: Expr::Variable("y".to_string()).into(),
                place_at: "ontop".to_string(),
                point: Some(
                    Expr::Point(Box::new(Expr::Integer(1)), Box::new(Expr::Integer(2))).into()
                )
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_place_without_point() {
    let code = "begin
    z:shape = place x ontop y;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "z");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::Place {
                base_shape: Expr::Variable("x".to_string()).into(),
                second_shape: Expr::Variable("y".to_string()).into(),
                place_at: "ontop".to_string(),
                point: None
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_rotate() {
    let code = "begin
    z:shape = rotate x by 5;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "z");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::Rotate {
                base_shape: Expr::Variable("x".to_string()).into(),
                factor: Expr::Integer(5).into()
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_scale() {
    let code = "begin
    z:shape = scale x by 5;
    ";
    let program = program::Program::new(&code.to_string()).unwrap();

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "z");
        assert_eq!(declared_type, &Type::Shape);
        assert_eq!(
            value,
            &Expr::Scale {
                base_shape: Expr::Variable("x".to_string()).into(),
                factor: Expr::Integer(5).into()
            }
        );
    }
}
//---------------------------------------------------
//Tests for Declaration of types in Declaration Field
//---------------------------------------------------
#[test]
fn test_program_new_converts_ast_to_program_decl_int_with_default_value() {
    let code = "x: int = 1;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(value.as_ref().unwrap(), &Expr::Integer(1))
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_int_without_default_value() {
    let code = "x: int;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert!(value.is_none())
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_float_with_default_value() {
    let code = "x: float = 1.0;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(value.as_ref().unwrap(), &Expr::Float(1.0));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_float_without_default_value() {
    let code = "x: float;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert!(value.is_none());
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_bool_with_default_value_true() {
    let code = "x: bool = true;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(value.as_ref().unwrap(), &Expr::Boolean(true));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_bool_with_default_value_false() {
    let code = "x: bool = false;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(value.as_ref().unwrap(), &Expr::Boolean(false));
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_bool_without_default_value() {
    let code = "x: bool;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert!(value.is_none());
    }
}

#[test]
fn test_program_new_converts_ast_to_program_decl_color_without_default_value() {
    let code = "x: color;
    begin
    y: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Decl {
        name,
        declared_type,
        value,
    } = &program.decl_f[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Color);
        assert!(value.is_none());
    }
}

//Needs to be implemented first
//#[test]
//fn test_program_new_converts_ast_to_program_decl_color_with_default_value() {
//  let code = "x: color = (0,0,0,0);
//begin
//    y: int = 1;";
//
//   let program = program::Program::new(&code.to_string()).unwrap();
//
//   assert_eq!(program.decl_f.len(), 1);

//if let Stmt::Decl {
//  name,
//declared_type,
//value,
//} = &program.decl_f[0]
//{
//  assert_eq!(name, "x");
//assert_eq!(declared_type, &Type::Color);
//assert_eq!(value.as_ref().unwrap(), &Expr::Color(
//  Box::new(Expr::Integer(0)),
//Box::new(Expr::Integer(0)),
//            Box::new(Expr::Integer(0)),
//          Box::new(Expr::Integer(0))
//    ));
//}
//}

#[test]
fn test_program_new_converts_ast_to_program_decl_point() {}

//---------------------------------------------------
//Tests for other statements in Declaration Field
//---------------------------------------------------

#[test]
fn test_program_new_converts_ast_to_program_decl_import() {
    let code = "import x \"x.extension\";
    begin
    x: int = 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.decl_f.len(), 1);

    if let Stmt::Import { name, path } = &program.decl_f[0] {
        assert_eq!(name, "x");
        assert_eq!(path, "x.extension");
    }
}

//---------------------------------------
//Testing of arithmic operations
//---------------------------------------
#[test]
fn test_program_new_converts_ast_to_program_addition_int_int() {
    let code = "begin
    x: int = 1 + 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::Add
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_addition_float_float() {
    let code = "begin
    x: float = 1.0 + 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Float(1.0)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Add
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_addition_int_float() {
    let code = "begin
    x: float = 1 + 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Add
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_subtraction_int_int() {
    let code = "begin
    x: int = 1 - 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::Subtract
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_subtraction_float_float() {
    let code = "begin
    x: float = 1.0 - 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Float(1.0)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Subtract
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_subtraction_int_float() {
    let code = "begin
    x: float = 1 - 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Subtract
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_multiplication_int_int() {
    let code = "begin
    x: int = 1 * 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::Multiply
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_multiplication_float_float() {
    let code = "begin
    x: float = 1.0 * 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Float(1.0)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Multiply
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_multiplication_int_float() {
    let code = "begin
    x: float = 1 * 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Multiply
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_modulus_int_int() {
    let code = "begin
    x: int = 1 % 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::Modulus
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_modulus_float_float() {
    let code = "begin
    x: float = 1.0 % 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Float(1.0)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Modulus
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_modulus_int_float() {
    let code = "begin
    x: float = 1 % 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Modulus
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_divide_int_int() {
    let code = "begin
    x: int = 1 / 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Int);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::Divide
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_divide_float_float() {
    let code = "begin
    x: float = 1.0 / 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Float(1.0)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Divide
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_divide_int_float() {
    let code = "begin
    x: float = 1 / 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Float);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::Divide
            }
        );
    }
}

//---------------------------
//Tests of logical operations
//---------------------------

#[test]
fn test_program_new_converts_ast_to_program_ge_int_int() {
    let code = "begin
    x: bool = 1 >= 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::GreaterThanOrEquals
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_ge_float_float() {
    let code = "begin
    x: bool = 1.0 >= 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Float(1.0)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::GreaterThanOrEquals
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_ge_int_float() {
    let code = "begin
    x: bool = 1 >= 1.0;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Float(1.0)),
                operator: BinaryOperator::GreaterThanOrEquals
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_great_int_int() {
    let code = "begin
    x: bool = 1 > 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::GreaterThan
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_less_int_int() {
    let code = "begin
    x: bool = 1 < 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::LessThan
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_le_int_int() {
    let code = "begin
    x: bool = 1 <= 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::LessThanOrEquals
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_ne_int_int() {
    let code = "begin
    x: bool = 1 != 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::NotEquals
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_equals_int_int() {
    let code = "begin
    x: bool = 1 == 1;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Integer(1)),
                rhs: Box::new(Expr::Integer(1)),
                operator: BinaryOperator::Equals
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_log_and() {
    let code = "begin
    x: bool = true && false;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Boolean(true)),
                rhs: Box::new(Expr::Boolean(false)),
                operator: BinaryOperator::LogicalAnd
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_log_or() {
    let code = "begin
    x: bool = true || false;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::BinaryOperation {
                lhs: Box::new(Expr::Boolean(true)),
                rhs: Box::new(Expr::Boolean(false)),
                operator: BinaryOperator::LogicalOr
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_negate() {
    let code = "begin
    x: bool = !true;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::VarDecl {
        name,
        declared_type,
        value,
    } = &program.stmts[0]
    {
        assert_eq!(name, "x");
        assert_eq!(declared_type, &Type::Bool);
        assert_eq!(
            value,
            &Expr::UnaryOperation {
                operator: UnaryOperator::Negate,
                expr: Box::new(Expr::Boolean(true))
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_function_decl_with_return_type() {
    let code = "begin
    func(x: int): int -> {
        f: int = 1;
        return f;
    }";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::FuncDecl {
        name,
        return_type,
        parameters,
        statements,
    } = &program.stmts[0]
    {
        assert_eq!(name, "func");
        assert_eq!(return_type, &Type::Int);
        assert_eq!(parameters, &vec![("x".into(), Type::Int)]);
        assert_eq!(
            statements,
            &vec![
                Stmt::VarDecl {
                    name: "f".into(),
                    declared_type: Type::Int,
                    value: Expr::Integer(1)
                },
                Stmt::Return(Expr::Variable("f".into()))
            ]
        );
    }
}

//-----------------------------------
//Tests of draw in construction field
//-----------------------------------
#[test]
fn test_program_draw_without_point() {
    let code = "begin
    draw x;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::Draw { shape, point } = &program.stmts[0] {
        assert_eq!(shape, &Expr::Variable("x".to_string()));
        assert_eq!(point, &None);
    }
}

#[test]
fn test_program_draw_with_point() {
    let code = "begin
    draw x at (0,0);";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::Draw { shape, point } = &program.stmts[0] {
        assert_eq!(shape, &Expr::Variable("x".to_string()));
        assert_eq!(
            point,
            &Some(Expr::Point(Expr::Integer(0).into(), Expr::Integer(0).into()).into())
        );
    }
}

//-----------------------------------
//Tests of assign in construction field
//-----------------------------------
#[test]
fn test_program_assign() {
    let code = "begin
    x = 10;";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::Assign { name, value } = &program.stmts[0] {
        assert_eq!(name, &"x".to_string());
        assert_eq!(value, &Expr::Integer(10));
    }
}

//-----------------------------------
//Tests of for in construction field
//-----------------------------------
#[test]
fn test_program_for_loop() {
    let code = "begin
    for i in 1 to 10 {
        x = i;
    }";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::For {
        counter,
        from,
        to,
        body,
    } = &program.stmts[0]
    {
        assert_eq!(counter, &"i".to_string());
        assert_eq!(from, &Expr::Integer(1));
        assert_eq!(to, &Expr::Integer(10));
        assert_eq!(
            body,
            &vec![Stmt::Assign {
                name: "x".to_string(),
                value: Expr::Variable("i".to_string())
            }]
        );
    }
}

//-----------------------------------
//Tests of fork in construction field
//-----------------------------------
#[test]
fn test_program_fork() {
    let code = "begin
    fork{
        (true) -> {x:int = 2;}
    }";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::Fork { branches: branch, otherwise } = &program.stmts[0] {
        assert_eq!(otherwise, &None);
        assert_eq!(
            branch,
            &vec![(
                Expr::Boolean(true),
                vec![Stmt::VarDecl {
                    name: "x".to_string(),
                    declared_type: Type::Int,
                    value: Expr::Integer(2)
                }]
            )]
        );
    }
}

#[test]
fn test_program_fork_otherwise() {
    let code = "begin
    fork{
        (true) -> {x:int = 2;}
        (otherwise) -> {y:int = 3;}
    }";

    let program = program::Program::new(&code.to_string()).unwrap();

    assert_eq!(program.stmts.len(), 1);

    if let Stmt::Fork { branches: branch, otherwise } = &program.stmts[0] {
        assert_eq!(
            otherwise,
            &Some(vec![Stmt::VarDecl {
                name: "y".to_string(),
                declared_type: Type::Int,
                value: Expr::Integer(3)
            }])
        );
        assert_eq!(
            branch,
            &vec![(
                Expr::Boolean(true),
                vec![Stmt::VarDecl {
                    name: "x".to_string(),
                    declared_type: Type::Int,
                    value: Expr::Integer(2)
                }]
            )]
        );
    }
}
