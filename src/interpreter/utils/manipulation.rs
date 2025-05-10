use crate::interpreter::data_types::point::Point;
use crate::interpreter::value::Value;
use crate::interpreter::{
    data_types::figurearray::FigureArray,
    errors,
};
use std::error::Error;

pub fn scale(shape: FigureArray, factor: Value) -> Result<FigureArray, Box<dyn Error>> {
    Ok(FigureArray::new())
}

/*pub fn scale(shape: FigureArray, factor: Value) -> Result<FigureArray, Box<dyn Error>> {
    let mut shape = shape.clone();
    let origin_x = shape
        .get_mut_figures()
        .iter()
        .map(|l| l.get_min_x())
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or_else(|| Box::new(errors::MaxCanNotBeFound))?;
    let origin_y = shape
        .get_mut_figures()
        .iter()
        .map(|l| l.get_max_y())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .ok_or_else(|| Box::new(errors::MinCanNotBeFound))?;

    let factor = match factor {
        Value::Integer(i) => i as f64,
        Value::Float(f) => f,
        _ => unreachable!(),
    };

        shape.get_mut_figures().iter_mut().for_each(|fig| {
            fig.get_mut_lines().iter_mut().for_each(|line| {
                line.get_mut_points().iter_mut().for_each(|point| { 
                    let x = point.get_x();
                    let y = point.get_y();

                    let dist_origin_x = x.get_float().unwrap() - origin_x;
                    let dist_origin_y =  origin_y - y.get_float().unwrap();
                    
                    point.set_x(match *x {
                        Value::Integer(_) =>  Value::Float(dist_origin_x * factor + origin_x).into(),
                        Value::Float(_) =>  Value::Float(dist_origin_x * factor + origin_x).into(),
                        _ => unreachable!()
                    });
                    point.set_y(match *point.get_y() {
                        Value::Integer(_) =>  Value::Float(dist_origin_y * factor + origin_y).into(),
                        Value::Float(_) =>  Value::Float(dist_origin_y * factor + origin_y).into(),
                        _ => unreachable!()
                    });
                });
            
            });
        });

    Ok(shape)
}*/


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
