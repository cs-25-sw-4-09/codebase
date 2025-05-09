use crate::{interpreter::errors, program::expression::Expr};
use std::{collections::HashMap, default, error::Error};
use super::point::Point;
use crate::interpreter::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Figure {
    lines: Vec<Line>,
    attributes: HashMap<String, Value> 
}

impl From<Vec<Line>> for Figure {
    fn from(value: Vec<Line>) -> Self {
        let mut fig = Figure::new();
        fig.push_lines(value);
        fig
    }
}

impl Figure {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            attributes: HashMap::new()
        }
    }

    pub fn set_attribute(&mut self, attribute: (String, Value)) {
        self.attributes.insert(attribute.0, attribute.1);
    }

    pub fn push_points(&mut self, ps: Vec<Point>) {
        self.lines.push(Line::Curved(ps));
    }

    pub fn push_line_after(&mut self, line: Line) {
        self.lines.push(line);
    }
    pub fn push_line_before(&mut self, line: Line) {
        self.lines.insert(0, line);
    }

    pub fn push_lines(&mut self, lines: Vec<Line>) {
        lines.into_iter().for_each(|line| self.lines.push(line));
    }

    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn get_mut_line(&mut self, idx: usize) -> Option<&mut Line> {
        self.lines.get_mut(idx)
    }

    pub fn get_mut_lines(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }

    pub fn get_max_x(&self) -> f64 {
        let mut max_x = self.lines.iter()
       .flat_map(|line| line.get_points())
       .filter_map(|point| match point.x(){
        Value::Float(f) => Some(*f),
        Value::Integer(i) => Some(*i as f64),
        _ => None,
       }).max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0f64);
       max_x
    }

    pub fn get_min_x(&self) -> f64 { 
        let mut min_x = self.lines.iter()
       .flat_map(|line| line.get_points())
       .filter_map(|point| match point.x(){
        Value::Float(f) => Some(*f),
        Value::Integer(i) => Some(*i as f64),
        _ => None,
       }).min_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0f64);
       min_x
    }

    pub fn get_height(&self) -> f64 {
       self.get_max_x() - self.get_min_x()
    }

    pub fn get_max_y(&self) -> f64 {
        let mut max_y = self.lines.iter()
        .flat_map(|line| line.get_points())
        .filter_map(|point| match point.y(){
        Value::Float(f) => Some(*f),
        Value::Integer(i) => Some(*i as f64),
         _ => None,
        }).max_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0f64);
        max_y
    }

    pub fn get_min_y(&self) -> f64 {
        let mut min_y = self.lines.iter()
        .flat_map(|line| line.get_points())
        .filter_map(|point| match point.y(){
        Value::Float(f) => Some(*f),
        Value::Integer(i) => Some(*i as f64),
         _ => None,
        }).min_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0f64);

        min_y
    }

    pub fn get_width(&self) -> f64 {
        self.get_max_y() - self.get_min_y()
     }

     pub fn get_last_line(&mut self) -> Result<&mut Line, Box<dyn Error>> {
         self.lines.last_mut().ok_or_else(|| errors::NoLinesInFigure.into())
     }
     pub fn get_first_line(&mut self) -> Result<&mut Line, Box<dyn Error>> {
         self.lines.first_mut().ok_or_else(|| errors::NoLinesInFigure.into())
     }
 
     pub fn pop_first_line(&mut self) -> Result<Line, Box<dyn Error>>{
         if self.lines.is_empty() {
             Err(errors::NoLinesInFigure.into())
         } else {
             Ok(self.lines.remove(0))
         }
     }
     pub fn pop_last_line(&mut self) -> Result<Line, Box<dyn Error>>  {
         self.lines.pop().ok_or_else(|| errors::NoLinesInFigure.into())
     }


}

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
}

