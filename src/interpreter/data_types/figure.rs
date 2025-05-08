use crate::program::expression::Expr;
use std::collections::HashMap;
use super::point::Point;
use crate::interpreter::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Figure {
    lines: Vec<Line>,
    attributes: HashMap<String, Box::<Expr>> 
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

    pub fn push_points(&mut self, ps: Vec<Point>) {
        self.lines.push(Line::from(ps));
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

    pub fn get_max_x(&self) -> i64 {
        let mut max_x = self.lines.iter()
       .flat_map(|line| line.get_points())
       .filter_map(|point| match point.x(){
        Value::Integer(i) => Some(*i),
        _ => None,
       }).max().unwrap_or(0);
       max_x
    }

    pub fn get_min_x(&self) -> i64 { 
        let mut min_x = self.lines.iter()
       .flat_map(|line| line.get_points())
       .filter_map(|point| match point.x(){
        Value::Integer(i) => Some(*i),
        _ => None,
       }).min().unwrap_or(0);
       min_x
    }

    pub fn get_height(&self) -> i64 {
       self.get_max_x() - self.get_min_x()
    }

    pub fn get_max_y(&self) -> i64 {
        let mut max_y = self.lines.iter()
        .flat_map(|line| line.get_points())
        .filter_map(|point| match point.y(){
         Value::Integer(i) => Some(*i),
         _ => None,
        }).max().unwrap_or(0);
        max_y
    }

    pub fn get_min_y(&self) -> i64 {
        let mut min_y = self.lines.iter()
        .flat_map(|line| line.get_points())
        .filter_map(|point| match point.y(){
         Value::Integer(i) => Some(*i),
         _ => None,
        }).min().unwrap_or(0);
        min_y
    }
    

    pub fn get_width(&self) -> i64 {
        self.get_max_y() - self.get_min_y()
     }


}

#[derive(Debug, PartialEq, Clone)]
pub struct Line(Vec<Point>);
impl Line {
    pub fn get_points(&self) -> &Vec<Point> { &self.0 }
    pub fn push_point(&mut self, val: Point) { self.0.push(val); }
}

impl From<(Point, Point)> for Line {
    fn from(value: (Point, Point)) -> Self {
        Self(vec![value.0, value.1])
    }
}

impl From<Vec<Point>> for Line {
    fn from(value: Vec<Point>) -> Self {
        Self(value)
    }
}