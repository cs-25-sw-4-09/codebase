use super::{figurearray::FigureArray, point::Point, super::Value};

#[derive(PartialEq, Debug)]
pub enum Direction {
    Top, 
    Bottom, 
    Right,
    Left, 
    Ontop,
    Center
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
            Direction::Center => {
                let cx1 = s1.min_x()-s1.max_x();
                let cy1 = s1.min_y()-s1.max_y();
                let cx2 = s2.min_x()-s2.max_x();
                let cy2 = s2.min_y()-s2.max_y();
                
                (Value::Float((cx1-cx2).get_float().unwrap() / 2.0), Value::Float((cy2-cy1).get_float().unwrap() / 2.0))
            },
        }.into()
    }
}
