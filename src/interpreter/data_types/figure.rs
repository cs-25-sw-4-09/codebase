use crate::program::expression::Expr;
use std::collections::HashMap;
use super::point::Point;

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

    pub fn push_line(&mut self, ps: Vec<Point>) {
        self.lines.push(Line::new(ps));
    }
    pub fn push_lines(&mut self, lines: Vec<Line>) {
        lines.into_iter().for_each(|line| self.lines.push(line));
    }

    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line(Vec<Point>);
impl Line {
    pub fn new(value: Vec<Point>) -> Self { Self(value) }
    pub fn get_points(&self) -> &Vec<Point> { &self.0 }
}
