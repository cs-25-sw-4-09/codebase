use crate::interpreter::{data_types::line::Line, value::Value};

pub mod errors;
pub mod generator;
pub mod svggenerator;
mod tests;

#[allow(dead_code)]
pub fn basic_line() -> Value {
    Value::Shape(
        vec![(
            vec![Line::Straight(vec![
                (Value::Integer(0), Value::Integer(0)).into(),
                (Value::Integer(1), Value::Integer(0)).into(),
            ])]
            .into(),
            vec![].into_iter().collect(),
        )
            .into()]
        .into(),
    )
}

pub fn basic_q() -> Value {
    Value::Shape(
        vec![(
            vec![Line::Straight(vec![
                (Value::Integer(0), Value::Integer(0)).into(),
                (Value::Integer(10), Value::Integer(0)).into(),
                (Value::Integer(10), Value::Integer(10)).into(),
            ])]
            .into(),
            vec![].into_iter().collect(),
        )
            .into()]
        .into(),
    )
}

pub fn basic_c() -> Value {
    Value::Shape(
        vec![(
            vec![Line::Straight(vec![
                (Value::Integer(0), Value::Integer(0)).into(),
                (Value::Float(10.0), Value::Integer(0)).into(),
                (Value::Integer(10), Value::Float(10.0)).into(),
                (Value::Integer(10), Value::Integer(20)).into(),
            ])]
            .into(),
            vec![].into_iter().collect(),
        )
            .into()]
        .into(),
    )
}

pub fn basic_line_with_stroke() -> Value {
    Value::Shape(
        vec![(
            vec![Line::Straight(vec![
                (Value::Integer(0), Value::Integer(0)).into(),
                (Value::Float(10.0), Value::Integer(0)).into(),
                (Value::Integer(10), Value::Float(10.0)).into(),
                (Value::Integer(10), Value::Integer(20)).into(),
            ])]
            .into(),
            vec![(
                "stroke".to_owned(),
                Value::Color(
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                )
                .into(),
            )]
            .into_iter()
            .collect(),
        )
            .into()]
        .into(),
    )
}

pub fn basic_line_with_thickness() -> Value {
    Value::Shape(
        vec![(
            vec![Line::Straight(vec![
                (Value::Integer(0), Value::Integer(0)).into(),
                (Value::Float(10.0), Value::Integer(0)).into(),
                (Value::Integer(10), Value::Float(10.0)).into(),
                (Value::Integer(10), Value::Integer(20)).into(),
            ])]
            .into(),
            vec![("thickness".to_owned(), Value::Integer(1).into())]
                .into_iter()
                .collect(),
        )
            .into()]
        .into(),
    )
}

pub fn basic_line_with_fill() -> Value {
    Value::Shape(
        vec![(
            vec![Line::Straight(vec![
                (Value::Integer(0), Value::Integer(0)).into(),
                (Value::Float(10.0), Value::Integer(0)).into(),
                (Value::Integer(10), Value::Float(10.0)).into(),
                (Value::Integer(0), Value::Integer(0)).into(),
            ])]
            .into(),
            vec![(
                "fill".to_owned(),
                Value::Color(
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                )
                .into(),
            )]
            .into_iter()
            .collect(),
        )
            .into()]
        .into(),
    )
}
