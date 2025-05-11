use super::{figurearray::FigureArray, point::Point, super::Value};

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
