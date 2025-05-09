use super::point::Point;
use crate::interpreter::value::Value;
use crate::{interpreter::errors, program::expression::Expr};
use std::{collections::HashMap, error::Error};

#[derive(Debug, PartialEq, Clone)]
pub struct Figure {
    lines: Vec<Line>,
    attributes: HashMap<String, Value>,
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
            attributes: HashMap::new(),
        }
    }

    pub fn set_attribute(&mut self, attribute: (String, Value)) {
        self.attributes.insert(attribute.0, attribute.1);
    }

    pub fn get_attributes(&mut self) -> &HashMap<String, Value> {
        &self.attributes
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

    pub fn get_max_x(&self) -> i64 {
        let mut max_x = self
            .lines
            .iter()
            .flat_map(|line| line.get_points())
            .filter_map(|point| match point.x() {
                Value::Integer(i) => Some(*i),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        max_x
    }

    pub fn get_min_x(&self) -> i64 {
        let mut min_x = self
            .lines
            .iter()
            .flat_map(|line| line.get_points())
            .filter_map(|point| match point.x() {
                Value::Integer(i) => Some(*i),
                _ => None,
            })
            .min()
            .unwrap_or(0);
        min_x
    }

    pub fn get_height(&self) -> i64 {
        self.get_max_x() - self.get_min_x()
    }

    pub fn get_max_y(&self) -> i64 {
        let mut max_y = self
            .lines
            .iter()
            .flat_map(|line| line.get_points())
            .filter_map(|point| match point.y() {
                Value::Integer(i) => Some(*i),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        max_y
    }

    pub fn get_min_y(&self) -> i64 {
        let mut min_y = self
            .lines
            .iter()
            .flat_map(|line| line.get_points())
            .filter_map(|point| match point.y() {
                Value::Integer(i) => Some(*i),
                _ => None,
            })
            .min()
            .unwrap_or(0);

        min_y
    }

    pub fn get_width(&self) -> i64 {
        self.get_max_y() - self.get_min_y()
    }

    pub fn get_last_line(&mut self) -> Result<&mut Line, Box<dyn Error>> {
        self.lines
            .last_mut()
            .ok_or_else(|| errors::NoLinesInFigure.into())
    }
    pub fn get_first_line(&mut self) -> Result<&mut Line, Box<dyn Error>> {
        self.lines
            .first_mut()
            .ok_or_else(|| errors::NoLinesInFigure.into())
    }

    pub fn pop_first_line(&mut self) -> Result<Line, Box<dyn Error>> {
        if self.lines.is_empty() {
            Err(errors::NoLinesInFigure.into())
        } else {
            Ok(self.lines.remove(0))
        }
    }
    pub fn pop_last_line(&mut self) -> Result<Line, Box<dyn Error>> {
        self.lines
            .pop()
            .ok_or_else(|| errors::NoLinesInFigure.into())
    }

    pub fn is_closed(mut self) -> Result<bool, Box<dyn Error>> {
        let first_point = {
            let line_ref = self.get_first_line()?;
            let point_ref = line_ref.get_first_point()?;
            point_ref.clone()
        };

        let last_point = {
            let line_ref = self.get_last_line()?;
            let point_ref = line_ref.get_last_point()?;
            point_ref.clone()
        };

        Ok(first_point == last_point)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    Straight(Vec<Point>),
    Curved(Vec<Point>),
}
impl Line {
    pub fn get_points(&self) -> &Vec<Point> {
        match self {
            Line::Straight(points) | Line::Curved(points) => &points,
        }
    }
    pub fn push_point(&mut self, val: Point) {
        match self {
            Line::Straight(points) | Line::Curved(points) => points.push(val),
        }
    }
    pub fn get_last_point(&self) -> Result<&Point, Box<dyn Error>> {
        match self {
            Line::Straight(points) | Line::Curved(points) => {
                points.last().ok_or_else(|| errors::NoLinesInFigure.into())
            }
        }
    }
    pub fn get_first_point(&self) -> Result<&Point, Box<dyn Error>> {
        match self {
            Line::Straight(points) | Line::Curved(points) => {
                points.first().ok_or_else(|| errors::NoLinesInFigure.into())
            }
        }
    }
    pub fn insert_point_first(&mut self, p: Point) {
        match self {
            Line::Straight(points) | Line::Curved(points) => points.insert(0, p),
        }
    }
    pub fn insert_point_last(&mut self, p: Point) {
        match self {
            Line::Straight(points) | Line::Curved(points) => points.push(p),
        }
    }
}
