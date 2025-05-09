use crate::interpreter::data_types::point::Point;
use crate::interpreter::value::Value;
use crate::{
    interpreter::{
        data_types::{figure::Figure, figurearray::FigureArray, },
        errors,
    },
    program::{expression::Expr, operators::pathoperator::PathOperator},
};
use std::error::Error;

pub fn scale(mut shape: FigureArray, factor: Value) -> Result<(), Box<dyn Error>> {
    let origin_x = shape
        .get_mut_figures()
        .iter()
        .map(|l| l.get_max_x())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or_else(|| Box::new(errors::MaxCanNotBeFound))?;
    let origin_y = shape
        .get_mut_figures()
        .iter()
        .map(|l| l.get_min_y())
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or_else(|| Box::new(errors::MinCanNotBeFound))?;

    let factor = match factor {
        Value::Integer(i) => i as f64,
        Value::Float(f) => f,
        _ => unreachable!(),
    };

        shape.get_mut_figures().iter_mut().for_each(|fig| {
            fig.get_mut_lines().iter_mut().for_each(|line| {
                line.get_mut_points().iter_mut().for_each(|point| { 
                    let dist_origin_x = point.x().get_float().unwrap() - origin_x;
                    let dist_origin_y = point.y().get_float().unwrap() - origin_y;
                    
                    point.x = match *point.x {
                        Value::Integer(_) =>  Value::Float(dist_origin_x * factor).into(),
                        Value::Float(_) =>  Value::Float(dist_origin_x * factor).into(),
                        _ => unreachable!()
                    };
                    point.y = match *point.y {
                        Value::Integer(_) =>  Value::Float(dist_origin_y * factor).into(),
                        Value::Float(_) =>  Value::Float(dist_origin_y * factor).into(),
                        _ => unreachable!()
                    };
                });
            
            });
        });

    Ok(())
}


//todo: 
/*
- Logic for top_left
- Logic for dir_offset
- Logic for place_point 
- Logic for Point addition and Subtraction
- Add fig_new to s1
*/
pub fn place(mut s1: FigureArray, mut s2: FigureArray, offset: Point, direction: Direction) -> FigureArray {
    let p1_top_left = s1.get_top_left();
    let p_offset = offset + s2.get_top_left() + direction.offset(&s1,&s2);

    let fig_new = s2.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points().iter_mut().for_each(|point| {
                *point = place_point(&p1_top_left, point, &p_offset)
            });
        })
    });


    todo!()
}

fn place_point(point_top_left: &Point, point: &Point, offset: &Point) -> Point {
    todo!()
}

pub enum Direction {
    Top, 
    Bottom, 
    Right,
    Left, 
    Ontop
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        use Direction::*;
        match value {
            "top" => Top,
            "bottom" => Bottom,
            "right" => Right,
            "left" => Left,
            "ontop" => Ontop,
            _ => unreachable!()
        }
    }
}

impl Direction {
    fn offset(&self, s1: &FigureArray, s2: &FigureArray) -> Point {
        match self {
            Direction::Top => todo!(),
            Direction::Bottom => todo!(),
            Direction::Right => todo!(),
            Direction::Left => todo!(),
            Direction::Ontop => todo!(),
        }
    }
}
