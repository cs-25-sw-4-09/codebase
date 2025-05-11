use crate::interpreter::data_types::point::Point;
use crate::interpreter::value::Value;
use crate::interpreter::data_types::{figurearray::FigureArray, direction::Direction};
use core::f64;
use std::error::Error;


/*************************  Scale ****************************/
pub fn scale(mut shape: FigureArray, factor: Value) -> Result<FigureArray, Box<dyn Error>> {
    let top_left = shape.get_top_left();
    shape.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points().iter_mut().for_each(|point| { 
                *point = scale_point(point, &top_left, &factor);
            });
        });
    });
    Ok(shape)
}

pub fn scale_point(p: &Point, top_left: &Point, factor: &Value) -> Point {
    let dist: Point = (
        p.get_x() - top_left.get_x(), //x
        -(top_left.get_y() - p.get_y()) //y
    ).into();
    &(dist * factor) + top_left
}


/*************************  Place ****************************/
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
                *point = place_point_at(&top_left, point, &p)
            });
        })
    });
    s
}

pub fn place_point_at(point_top_left: &Point, point: &Point, offset: &Point) -> Point { &(point - point_top_left) + offset }

/*************************  Rotate ****************************/
pub fn rotate(mut s: FigureArray, rotate_by: Value) -> FigureArray {
    //todo: mangler stadig calc this
    let rotate_around = Point::from((0,0));
    s.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points().iter_mut().for_each(|point| {
                *point = rotate_point(point, &rotate_around, &rotate_by)
            });
        })
    });
    s
}

pub fn rotate_point(p: &Point, rotate_around: &Point, degrees: &Value) -> Point {
    let dist = p - rotate_around;
    let (cos_theta, sin_theta) = calc_sin_theta(degrees);
    let transformed_dist: Point = (
        dist.get_x() * &cos_theta + dist.get_y() * &sin_theta, //x
        dist.get_x() * &sin_theta - dist.get_y() * &cos_theta  //y
    ).into();
    &transformed_dist + rotate_around
}

/**Gives cleaner outpus for sin and cos*/
fn calc_sin_theta(degrees: &Value) -> (Value, Value) {
    use Value::*;
    match degrees {
        Integer(90) | Float(90.) => (0.into(), 1.into()),
        Integer(60) | Float(60.) => (0.5.into(), 0.87.into()),
        Integer(45) | Float(45.) => (0.71.into(), 0.71.into()),
        Integer(30) | Float(30.) => (0.87.into(), 0.5.into()), 
        Integer(0) | Float(0.) => (1.into(), 0.into()),
        val => {
            let radians = val * &(f64::consts::PI / 180.).into();
            let (cos, sin) = (cos(&radians), sin(&radians));
            //todo: add snap_zero check
            (cos, sin)
        }
    }
    
}

pub fn cos(theta: &Value) -> Value {
    match theta {
        Value::Integer(v) => (*v as f64).cos().into(),
        Value::Float(v) => v.cos().into(),
        _ => unreachable!()
    }   
}

pub fn sin(theta: &Value) -> Value {
     match theta {
        Value::Integer(v) => (*v as f64).sin().into(),
        Value::Float(v) => v.sin().into(),
        _ => unreachable!()
    }  
}