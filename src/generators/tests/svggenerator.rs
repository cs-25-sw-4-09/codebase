use std::collections::HashMap;

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
        expression::Expr, operators::{
            binaryoperator::BinaryOperator, pathoperator::PathOperator, polyoperator::PolyOperator,
            unaryoperator::UnaryOperator,
        }, program::Program, statement::Stmt, r#type::Type
    },
};

#[test]
fn line() {

}