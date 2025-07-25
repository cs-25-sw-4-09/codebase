
#[cfg(test)]
mod expression;

#[cfg(test)]
mod statement;

#[cfg(test)]
mod manipulation;

#[cfg(test)]
mod data_types;

#[cfg(test)]
mod whole_program_test;
pub mod valid_programs;

use crate::interpreter::data_types::line::Line;
use super::value::Value;
#[allow(dead_code)]
pub fn basic_square() -> Value {
    Value::Shape(
        vec![(
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(1), Value::Integer(0)).into()
                    ]
                ),
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(0)).into(),
                    (Value::Integer(1), Value::Integer(1)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(1)).into(),
                    (Value::Integer(0), Value::Integer(1)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(1)).into(),
                    (Value::Integer(0), Value::Integer(0)).into()
                ])
            ].into(),
            vec![(
                "fill".to_owned(),
                Value::Color(
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into()
                ).into()
            )]
            .into_iter()
            .collect()
        ).into()].into(),
    )
}

#[allow(dead_code)]
pub fn basic_triangle() -> Value {
    Value::Shape(
        vec![(
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(2), Value::Integer(0)).into()
                    ]
                ),
                Line::Straight(vec![
                    (Value::Integer(2), Value::Integer(0)).into(),
                    (Value::Integer(1), Value::Integer(2)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(2)).into(),
                    (Value::Integer(0), Value::Integer(0)).into()
                ]),
            ].into(),
            vec![(
                "fill".to_owned(),
                Value::Color(
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into()
                ).into()
            )]
            .into_iter()
            .collect()
        ).into()].into(),
    )
}

#[allow(dead_code)]
pub fn basic_house() -> Value {
    Value::Shape(
        vec![(
            //Square
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(1), Value::Integer(0)).into()
                    ]
                ),
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(0)).into(),
                    (Value::Integer(1), Value::Integer(1)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(1)).into(),
                    (Value::Integer(0), Value::Integer(1)).into()
                ]),
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(1)).into(),
                    (Value::Integer(0), Value::Integer(0)).into()
                ])
            ].into(),
            vec![(
                "fill".to_owned(),
                Value::Color(
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into()
                ).into()
            )]
            .into_iter()
            .collect()
        ).into(),
        //Triangle
        (vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(1)).into(),
                    (Value::Integer(1), Value::Integer(1)).into()
                    ]
                ),
                Line::Straight(vec![
                    (Value::Integer(1), Value::Integer(1)).into(),
                    (Value::Float(0.5), Value::Integer(2)).into()
                ]),
                Line::Straight(vec![
                    (Value::Float(0.5), Value::Integer(2)).into(),
                    (Value::Integer(0), Value::Integer(1)).into()
                ])
            ].into(),
            vec![(
                "fill".to_owned(),
                Value::Color(
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into(),
                    Value::Integer(255).into()
                ).into()
            )]
            .into_iter()
            .collect()
        ).into(),
        
        
        
        
        ].into(),
    )
}