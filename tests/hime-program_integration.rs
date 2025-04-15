use std::collections::HashMap;

use codebase::{
    program::{
        expression::Expr, operators::binaryoperator::BinaryOperator, program, r#type::Type,
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
                name: "dummy".into(),
                args: HashMap::from([
                    ("x".to_string(), Expr::Integer(1)),
                    ("b".to_string(), Expr::Boolean(true))
                ])
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
                name: "dummy".into(),
                args: HashMap::from([])
            }
        );
    }
}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_path() {}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_polygon() {}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_string() {}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_color() {}

#[test]
fn test_program_new_converts_ast_to_program_var_decl_point() {}

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
fn test_program_new_converts_ast_to_program_decl_shape_without_default_value() {}

#[test]
fn test_program_new_converts_ast_to_program_decl_path() {}

#[test]
fn test_program_new_converts_ast_to_program_decl_polygon() {}

#[test]
fn test_program_new_converts_ast_to_program_decl_string() {}

#[test]
fn test_program_new_converts_ast_to_program_decl_color() {}

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