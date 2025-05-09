use crate::{
    interpreter::{
        data_types::{figure::{Figure, Line}, figurearray::FigureArray, point::Point}, environment::IEnvironment, InterpretE
    }, 
    program::{expression::Expr, operators::pathoperator::PathOperator}};
use crate::interpreter::value::Value;


/*pub fn scale(shape: Figure, factor: Value) { 
    let origin_x = shape.get_min_x() as f64;
    let origin_y = shape.get_max_y() as f64;

    for line in shape() {
        for point in line {

        }
    }
    let lefttop: Vec<i64> = vec![shape.get_max_x(), shape.get_min_y()];
    shape.get_lines().iter().map(|l| l.get_points() - lefttop[1])
}
*/

//todo: 
/*
- Logic for top_left
- Logic for width
- Logic for height
*/
pub fn place(s1: FigureArray, mut s2: FigureArray, offset: Point, direction: Direction) -> FigureArray {
    //s1 is placed in relation to s2, so the updated s1 is added to s2
    let p_offset = offset + s2.get_top_left() + direction.offset(&s1,&s2);
    let fig_new = place_shape_at(s1, p_offset);
    s2.push_figures(fig_new);
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

fn place_point_at(point_top_left: Point, point: Point, offset: Point) -> Point { (point_top_left - point) + offset }

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
            Direction::Top => (Value::Integer(0), s1.height()),
            Direction::Bottom => (Value::Integer(0), -s2.height()),
            Direction::Right => (-s1.width(), Value::Integer(0)),
            Direction::Left => (-s2.width(), Value::Integer(0)),
            Direction::Ontop => (Value::Integer(0), Value::Integer(0)),
        }.into()
    }
}