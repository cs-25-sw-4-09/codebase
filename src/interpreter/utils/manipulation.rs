use crate::interpreter::data_types::point::Point;
use crate::interpreter::data_types::{direction::Direction, figurearray::FigureArray};
use crate::interpreter::value::Value;
use core::f64;
use std::error::Error;

/*************************  Place ****************************/
pub fn place(
    s1: FigureArray,
    mut s2: FigureArray,
    offset: Point,
    direction: Direction,
) -> FigureArray {
    //s1 is placed in relation to s2, so the updated s1 is added to s2
    let p_offset: Point = offset + s2.get_top_left() + direction.offset(&s1, &s2);

    let fig_new = place_shape_at(s1, p_offset);
    s2.extend(fig_new);
    s2
}

pub fn place_shape_at(mut s: FigureArray, p: Point) -> FigureArray {
    let top_left = s.get_top_left();
    s.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points()
                .iter_mut()
                .for_each(|point| *point = place_point_at(&top_left, point, &p));
        })
    });
    s
}

pub fn place_point_at(point_top_left: &Point, point: &Point, offset: &Point) -> Point {
    &(point - point_top_left) + offset
}

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
    &((p - top_left) * factor) + top_left
}

/*************************  Rotate ****************************/
pub fn rotate(mut s: FigureArray, rotate_by: Value) -> FigureArray {
    let rotate_around = s.get_center();
    //Convert degrees into radians
    let theta = &rotate_by * &(f64::consts::PI / 180.).into();
    s.get_mut_figures().iter_mut().for_each(|fig| {
        fig.get_mut_lines().iter_mut().for_each(|line| {
            line.get_mut_points()
                .iter_mut()
                .for_each(|point| *point = rotate_point(point, &rotate_around, &theta));
        })
    });
    s
}

pub fn rotate_point(p: &Point, rotate_around: &Point, theta: &Value) -> Point {
    let dist = p - rotate_around;
    let (cos_theta, sin_theta) = calc_sin_theta(theta);
    let transformed_dist: Point = (
        dist.get_x() * &cos_theta + dist.get_y() * &sin_theta, //x
        -(dist.get_x() * &sin_theta) + dist.get_y() * &cos_theta, //y
    )
        .into();
    &transformed_dist + rotate_around
}

/**Gives cleaner outputs for sin and cos*/
fn calc_sin_theta(theta: &Value) -> (Value, Value) {
    let (cos, sin) = (
        snap_zero(cos(&theta), 1e-10),
        snap_zero(sin(&theta), 1e-10),
    );
    (cos, sin)
}

/**This exists because floating point arithmetic is unprecise*/
fn snap_zero(x: Value, eps: f64) -> Value {
    match x {
        Value::Integer(_) => x,
        Value::Float(v) => {
            if v.abs() < eps {
                Value::Integer(0)
            } else {
                x
            }
        }
        _ => unreachable!(),
    }
}

fn cos(theta: &Value) -> Value {
    match theta {
        Value::Integer(v) => (*v as f64).cos().into(),
        Value::Float(v) => v.cos().into(),
        _ => unreachable!(),
    }
}

fn sin(theta: &Value) -> Value {
    match theta {
        Value::Integer(v) => (*v as f64).sin().into(),
        Value::Float(v) => v.sin().into(),
        _ => unreachable!(),
    }
}
