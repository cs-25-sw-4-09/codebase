use crate::interpreter::{data_types::line::Line, value::Value};

use super::super::utils::manipulation::*;
use super::*;

/****************************** Place *******************************/
#[test] 
fn create_direction_enum() {
    assert_eq!(Direction::Ontop, Direction::from("ontop"));
    assert_eq!(Direction::Top, Direction::from("top"));
    assert_eq!(Direction::Bottom, Direction::from("bottom"));
    assert_eq!(Direction::Left, Direction::from("left"));
    assert_eq!(Direction::Right, Direction::from("right"));
} 

#[test]
fn direction_offset() {
    let i1 = basic_square().get_shape().unwrap();
    let i2 = basic_triangle().get_shape().unwrap();

    //Square is being placed in relation to the triangle
    //Should return the height of the square in the y-coordinate
    let i3 = Direction::from("top").offset(&i1, &i2);
    assert_eq!(i1.height(), Value::Integer(1));
    assert_eq!(i3, (Value::Integer(0), Value::Integer(1)).into());

    //Should be (0,0)
    let i3 = Direction::from("ontop").offset(&i1, &i2);
    assert_eq!(i3, (Value::Integer(0), Value::Integer(0)).into());

    //Should return the negated height of the triangle in the y-coordinate as we wish to place the square under the triangle
    let i3 = Direction::from("bottom").offset(&i1, &i2);
    assert_eq!(i2.height(), Value::Float(2.));
    assert_eq!(i3, (Value::Integer(0), Value::Float(-2.)).into());

    //Should return the negated width of the square in the x-coordinate as we wish to place the square to the left of triangle
    let i3 = Direction::from("left").offset(&i1, &i2);
    assert_eq!(i1.width(), Value::Integer(1));
    assert_eq!(i3, (Value::Integer(-1), Value::Integer(0)).into());

    //Should return the negated width of the square in the x-coordinate as we wish to place the square to the left of triangle
    let i3 = Direction::from("right").offset(&i1, &i2);
    assert_eq!(i2.width(), Value::Integer(2));
    assert_eq!(i3, (Value::Integer(2), Value::Integer(0)).into());
}

#[test]
fn test_place_point() {
    let top_left = (0,0).into();
    let point = (2,0).into(); 
    let offset = (100,100).into();
    let i1 = place_point_at(top_left, point, offset);
    assert_eq!(i1, (102, 100).into());

    let top_left = (-1,-1).into();
    let point = (2,0).into(); 
    let offset = (100,100).into();
    let i1 = place_point_at(top_left, point, offset);
    assert_eq!(i1, (103, 101).into());

    let top_left = (5,5).into();
    let point = (100, 50).into(); 
    let offset = (100,100).into();
    let i1 = place_point_at(top_left, point, offset);
    assert_eq!(i1, (195, 145).into());
}

#[test]
fn test_place_shape() {
    let i1 = basic_square().get_shape().unwrap(); 
    let i2 = (50,50).into();

    let i3 = place_shape_at(i1, i2);
    assert_eq!(i3.get_figures()[0].get_lines().to_vec(), 
            vec![
                Line::Straight(vec![
                    (Value::Float(50.), Value::Float(49.)).into(),
                    (Value::Float(51.), Value::Float(49.)).into()
                    ]
                ),
                Line::Straight(vec![
                    (Value::Float(51.), Value::Float(49.)).into(),
                    (Value::Float(51.), Value::Float(50.)).into()
                ]),
                Line::Straight(vec![
                    (Value::Float(51.), Value::Float(50.)).into(),
                    (Value::Float(50.), Value::Float(50.)).into()
                ]),
                Line::Straight(vec![
                    (Value::Float(50.), Value::Float(50.)).into(),
                    (Value::Float(50.), Value::Float(49.)).into()
                ])
            ]
    );
}

#[test]
fn test_place_no_offset() {
    let i1 = basic_square().get_shape().unwrap(); 
    let i2 = basic_triangle().get_shape().unwrap();

    let i3 = place(i1, i2, (0,0).into(), Direction::Top);
    assert_eq!(i3.get_figures().iter().map(|fig| fig.get_lines().to_vec()).collect::<Vec<Vec<Line>>>(), 
        vec![
            //Triangle
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
            ],
            //New Square
            vec![
                Line::Straight(vec![
                    (Value::Float(0.), Value::Float(2.)).into(),
                    (Value::Float(1.), Value::Float(2.)).into()
                    ]
                ),
                Line::Straight(vec![
                    (Value::Float(1.), Value::Float(2.)).into(),
                    (Value::Float(1.), Value::Float(3.)).into()
                ]),
                Line::Straight(vec![
                    (Value::Float(1.), Value::Float(3.)).into(),
                    (Value::Float(0.), Value::Float(3.)).into()
                ]),
                Line::Straight(vec![
                    (Value::Float(0.), Value::Float(3.)).into(),
                    (Value::Float(0.), Value::Float(2.)).into()
                ])
            ]
        ]
    );
}

#[test]
fn test_place_with_offset() {
    todo!()
}

/*********************************************** Scale *****************************************/



/*********************************************** Rotate ****************************************/