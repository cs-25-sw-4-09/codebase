use crate::interpreter::data_types::point::Point;
use crate::interpreter::value::Value;
use crate::interpreter::{
    data_types::figurearray::FigureArray,
    errors,
};
use std::error::Error;


pub fn scale(mut shape: FigureArray, factor: Value) -> Result<FigureArray, Box<dyn Error>> {
    let top_left = shape.get_top_left();
    shape.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points().iter_mut().for_each(|point| { 
                *point = scale_point(point.clone(), top_left.clone(), factor.clone());
            });
        
        });
    });
    Ok(shape)
}

fn scale_point(p: Point, top_left: Point, factor: Value) -> Point {
    (top_left.clone() - p) * factor + top_left
}



pub fn place(s1: FigureArray, mut s2: FigureArray, offset: Point, direction: Direction) -> FigureArray {
    //s1 is placed in relation to s2, so the updated s1 is added to s2
    let p_offset = offset + s2.get_top_left() + direction.offset(&s1,&s2);
    let fig_new = place_shape_at(s1, p_offset);
    s2.extend(fig_new);
    s2
}

pub fn place_shape_at(mut s: FigureArray, p: Point) -> FigureArray {
    let top_left = s.get_top_left();
    s.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points().iter_mut().for_each(|point| {
                *point = place_point_at(top_left.clone(), point.clone(), p.clone())
            });
        })
    });
    s
}

pub fn place_point_at(point_top_left: Point, point: Point, offset: Point) -> Point { (point - point_top_left) + offset }

#[derive(PartialEq, Debug)]
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
    pub fn offset(&self, s1: &FigureArray, s2: &FigureArray) -> Point {
        match self {
            Direction::Top => (Value::Integer(0), s1.height()),
            Direction::Bottom => (Value::Integer(0), -s2.height()),
            Direction::Left => (-s1.width(), Value::Integer(0)),
            Direction::Right => (s2.width(), Value::Integer(0)),
            Direction::Ontop => (Value::Integer(0), Value::Integer(0)),
        }.into()
    }
}
