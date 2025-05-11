use crate::interpreter::{data_types::line::Line, value::Value};

pub mod generator;
pub mod errors;
pub mod svggenerator;
mod tests;


#[allow(dead_code)]
pub fn basic_line() -> Value {
    Value::Shape(
        vec![(
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(1), Value::Integer(0)).into()
                    ]
                )
            ].into(),
            vec![]
            .into_iter()
            .collect()
        ).into()].into(),
    )
}

pub fn basic_q() -> Value {
    Value::Shape(
        vec![(
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(10), Value::Integer(0)).into(),
                    (Value::Integer(10), Value::Integer(10)).into()
                    ]
                )
            ].into(),
            vec![]
            .into_iter()
            .collect()
        ).into()].into(),
    )
}


pub fn basic_c() -> Value {
    Value::Shape(
        vec![(
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(10), Value::Integer(0)).into(),
                    (Value::Integer(10), Value::Integer(10)).into(),
                    (Value::Integer(10), Value::Integer(20)).into()
                    ]
                )
            ].into(),
            vec![]
            .into_iter()
            .collect()
        ).into()].into(),
    )
}