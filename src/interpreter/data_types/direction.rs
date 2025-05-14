use super::{super::Value, figurearray::FigureArray, point::Point};

#[derive(PartialEq, Debug)]
pub enum Direction {
    Top,
    Bottom,
    Right,
    Left,
    Ontop,
    Center,
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
            "center" => Center,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    pub fn offset(&self, s1: &FigureArray, s2: &FigureArray) -> Point {
        match self {
            Direction::Top => (Value::Integer(0), s1.height()).into(),
            Direction::Bottom => (Value::Integer(0), -s2.height()).into(),
            Direction::Left => (-s1.width(), Value::Integer(0)).into(),
            Direction::Right => (s2.width(), Value::Integer(0)).into(),
            Direction::Ontop => (Value::Integer(0), Value::Integer(0)).into(),
            Direction::Center => {
                let v1 = &s1.get_top_left() - &s1.get_center();
                let v2 = &s2.get_top_left() - &s2.get_center();
                &v1 - &v2
            }
        }
        
    }
}
