use std::error::Error;

use crate::interpreter::errors;

use super::point::Point;

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    Straight(Vec<Point>),
    Curved(Vec<Point>)
}
impl Line {
    pub fn get_points(&self) -> &Vec<Point> { match self {
        Line::Straight(points) | 
        Line::Curved(points) => &points,
    }  }

    pub fn push_point(&mut self, val: Point) { 
        match self {
            Line::Straight(points) | 
            Line::Curved(points) => points.push(val),
        } 
    }
    pub fn get_last_point(&self) -> Result<&Point, Box<dyn Error>> {
        match self {
            Line::Straight(points) | 
            Line::Curved(points) => points.last().ok_or_else(|| errors::NoLinesInFigure.into())
        }
    }
    pub fn get_first_point(&self) -> Result<&Point, Box<dyn Error>> {
        match self {
            Line::Straight(points) | 
            Line::Curved(points) => points.first().ok_or_else(|| errors::NoLinesInFigure.into())
        }
    }
    pub fn insert_point_first(&mut self, p: Point) {
        match self {
            Line::Straight(points) | 
            Line::Curved(points) => points.insert(0, p),
        }
    }
    pub fn insert_point_last(&mut self, p: Point) {
        match self {
            Line::Straight(points) | 
            Line::Curved(points) => points.push(p)
        }
    }

    pub fn get_mut_points(&mut self) -> &mut Vec<Point> {
        match self {
            Line::Straight(points) |
            Line::Curved(points) => points,
        }
    }
}

