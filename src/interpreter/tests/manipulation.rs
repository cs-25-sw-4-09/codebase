use crate::interpreter::{data_types::{direction::Direction, line::Line}, value::Value};

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
    assert_eq!(i3, (0,1).into());

    //Should be (0,0)
    let i3 = Direction::from("ontop").offset(&i1, &i2);
    assert_eq!(i3, (0,0).into());

    //Should return the negated height of the triangle in the y-coordinate as we wish to place the square under the triangle
    let i3 = Direction::from("bottom").offset(&i1, &i2);
    assert_eq!(i2.height(), 2.into());
    assert_eq!(i3, (0,-2).into());

    //Should return the negated width of the square in the x-coordinate as we wish to place the square to the left of triangle
    let i3 = Direction::from("left").offset(&i1, &i2);
    assert_eq!(i1.width(), 1.into());
    assert_eq!(i3, (-1,0).into());

    //Should return the negated width of the square in the x-coordinate as we wish to place the square to the left of triangle
    let i3 = Direction::from("right").offset(&i1, &i2);
    assert_eq!(i2.width(), 2.into());
    assert_eq!(i3, (2,0).into());

    let i3 = Direction::from("center").offset(&i1, &i2);
    assert_eq!(i2.get_center(), (1.,1.).into());
    assert_eq!(i1.get_center(), (0.5,0.5).into());
    assert_eq!(i3, (0.5,-0.5).into());
}

#[test]
fn test_place_point() {
    let top_left = (0,0).into();
    let point = (2,0).into(); 
    let offset = (100,100).into();
    let i1 = place_point_at(&top_left, &point, &offset);
    assert_eq!(i1, (102, 100).into());

    let top_left = (-1,-1).into();
    let point = (2,0).into(); 
    let offset = (100,100).into();
    let i1 = place_point_at(&top_left, &point, &offset);
    assert_eq!(i1, (103, 101).into());

    let top_left = (5,5).into();
    let point = (100, 50).into(); 
    let offset = (100,100).into();
    let i1 = place_point_at(&top_left, &point, &offset);
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
                    (50,49).into(),
                    (51,49).into()
                    ]
                ),
                Line::Straight(vec![
                    (51,49).into(),
                    (51,50).into()
                ]),
                Line::Straight(vec![
                    (51,50).into(),
                    (50,50).into()
                ]),
                Line::Straight(vec![
                    (50,50).into(),
                    (50,49).into()
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
                    (0,2).into(),
                    (1,2).into()
                    ]
                ),
                Line::Straight(vec![
                    (1,2).into(),
                    (1,3).into()
                ]),
                Line::Straight(vec![
                    (1,3).into(),
                    (0,3).into()
                ]),
                Line::Straight(vec![
                    (0,3).into(),
                    (0,2).into()
                ])
            ]
        ]
    );
}


#[test]
fn test_place_with_offset() {
    let i1 = basic_square().get_shape().unwrap(); 
    let i2 = basic_triangle().get_shape().unwrap();

    let i3 = place(i1, i2, (0,2).into(), Direction::Top);
    assert_eq!(i3.get_figures().iter().map(|fig| fig.get_lines().to_vec()).collect::<Vec<Vec<Line>>>(), 
        vec![
            //Triangle
            vec![
                Line::Straight(vec![
                    (Value::Integer(0), Value::Integer(0)).into(),
                    (Value::Integer(2), Value::Integer(0)).into()
                ]),
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
                    (0,4).into(),
                    (1,4).into()   
                ]),
                Line::Straight(vec![
                    (1,4).into(),
                    (1,5).into()
                ]),
                Line::Straight(vec![
                    (1,5).into(),
                    (0,5).into()
                ]),
                Line::Straight(vec![
                    (0,5).into(),
                    (0,4).into()
                ])
            ]
        ]
    );
}

/*********************************************** Scale *****************************************/
#[test]
fn scale_operation() {
    let i1 = basic_square().get_shape().unwrap();
    let i2 = scale(i1, 2.into()).unwrap(); 

    assert_eq!(i2.get_figures()[0].get_lines().to_vec(), 
        vec![
            Line::Straight(vec![
                (0,-1).into(),
                (2,-1).into(),
            ]),
            Line::Straight(vec![
                (2,-1).into(),
                (2, 1).into(),
            ]),
            Line::Straight(vec![
                (2, 1).into(),
                (0,1).into(),
            ]),
            Line::Straight(vec![
                (0,1).into(),
                (0,-1).into(),
            ])
        ]
    )
}

#[test]
fn test_scale_point() {
    let i1 = (0,0).into();
    let i2 = (2,0).into(); 
    let i3 = 5.into();
    let i4 = scale_point(&i2,&i1,&i3);
    assert_eq!(i4, (10,0).into());

    let i1 = (0,0).into();
    let i2 = (-2,0).into(); 
    let i3 = 5.into();
    let i4 = scale_point(&i2,&i1,&i3);
    assert_eq!(i4, (-10,0).into());

    let i1 = (0,2).into(); 
    let i2 = (0,0).into();
    let i3 = 5.into();
    let i4 = scale_point(&i2,&i1,&i3);
    assert_eq!(i4, (0,-8).into());

    let i1 = (0,-2).into(); 
    let i2 = (0,0).into();
    let i3 = 5.into();
    let i4 = scale_point(&i1, &i2,&i3);
    assert_eq!(i4, (0,-10).into());

    let i1 = (0,2).into();
    let i2 = (2,0).into(); 
    let i3 = 5.into();
    let i4 = scale_point(&i2,&i1,&i3);
    assert_eq!(i4, (10,-8).into());
}


/*********************************************** Rotate ****************************************/
#[test]
fn rotate_point_test() {
    // Rotates the point (0,5) around (0,0) clockwise and counter-clockwise
    let i1 = (0,0).into();
    let i2 = (0,5).into();
    //Calculate Radians
    let i3 = (90. * std::f64::consts::PI / 180.).into();
    let i4 = rotate_point(&i2, &i1, &i3);
    assert_eq!(i4, (5.,0.).into());
    let i4 = rotate_point(&i4, &i1, &i3);
    assert_eq!(i4, (0.,-5.).into());
    let i4 = rotate_point(&i4, &i1, &i3);
    assert_eq!(i4, (-5.,0.).into());
    let i4 = rotate_point(&i4, &i1, &i3);
    assert_eq!(i4, (0.,5.).into());

    //Calculate Radians
    let i3 = (-90. * std::f64::consts::PI / 180.).into();
    let i4 = rotate_point(&i2, &i1, &i3);
    assert_eq!(i4, (-5.,0.).into());
    let i4 = rotate_point(&i4, &i1, &i3);
    assert_eq!(i4, (0.,-5.).into());
    let i4 = rotate_point(&i4, &i1, &i3);
    assert_eq!(i4, (5.,0.).into());
    let i4 = rotate_point(&i4, &i1, &i3);
    assert_eq!(i4, (0.,5.).into());
}


#[test]
fn rotate_test() {
    let square = basic_square().get_shape().unwrap();
    let i1 = rotate(square, 45.into());
    let epsilon = 1e-6;

    let rotated_lines = vec![
        Line::Straight(vec![(-0.20710678118654757, 0.5).into(), (0.5,-0.20710678118654757).into()]),
        Line::Straight(vec![(0.5,-0.20710678118654757).into(), (1.20710678118654757,0.5).into()]),
        Line::Straight(vec![(1.20710678118654757,0.5).into(), (0.5,1.20710678118654757).into()]),
        Line::Straight(vec![(0.5,1.20710678118654757).into(),(-0.20710678118654757, 0.5).into()])
    ];

    i1.get_figures()[0].get_lines().iter().zip(rotated_lines.iter()).for_each(|(l1,l2)| 
        l1.get_points().iter().zip(l2.get_points().iter()).for_each(
            |(p1, p2)| assert!(p1.approx_eq(p2, epsilon), "Point mismatch: {:?} vs {:?}", p1, p2)
        )
    ); 

    let triangle = basic_triangle().get_shape().unwrap();
    let i1 = rotate(triangle, (-90).into());
    let epsilon = 1e-6;

    let rotated_lines = vec![
        Line::Straight(vec![(2., 0.).into(), (2.,2.).into()]),
        Line::Straight(vec![(2., 2.).into(), (0., 1.).into()]),
        Line::Straight(vec![(0., 1.).into(), (2.,0.).into()]),
    ];

    i1.get_figures()[0].get_lines().iter().zip(rotated_lines.iter()).for_each(|(l1,l2)| 
        l1.get_points().iter().zip(l2.get_points().iter()).for_each(
            |(p1, p2)| assert!(p1.approx_eq(p2, epsilon), "Point mismatch: {:?} vs {:?}", p1, p2)
        )
    )    
}
