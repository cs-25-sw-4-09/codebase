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
- Logic for dir_offset
- Logic for place_point 
- Logic for Point addition and Subtraction
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